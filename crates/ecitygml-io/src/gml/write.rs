use crate::Error::InvalidFileExtension;
use crate::gml::write_impl::serialize;
use crate::{CitygmlFormat, Error};
use ecitygml_core::model::core::CityModel;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

/// `GmlWriter` writes CityGML datasets.
///
#[derive(Debug, Clone)]
pub struct GmlWriter<W: Write> {
    writer: W,
    format: CitygmlFormat,
    formatting: Formatting,
}

impl<W: Write> GmlWriter<W> {
    pub fn new(writer: W, format: CitygmlFormat) -> Self {
        Self {
            writer,
            format,
            formatting: Formatting::default(),
        }
    }

    pub fn with_formatting(mut self, formatting: Formatting) -> Self {
        self.formatting = formatting;
        self
    }

    pub fn finish(self, city_model: CityModel) -> Result<(), Error> {
        match self.format {
            CitygmlFormat::Gml => {
                let mut w = BufWriter::new(self.writer);
                serialize(&mut w, city_model, self.formatting)?;
            }
            CitygmlFormat::GmlZst => {
                let mut encoder = zstd::Encoder::new(BufWriter::new(self.writer), 9)?;
                serialize(&mut encoder, city_model, self.formatting)?;
                encoder.finish()?;
            }
            CitygmlFormat::GmlGz => {
                let mut encoder = flate2::write::GzEncoder::new(
                    BufWriter::new(self.writer),
                    flate2::Compression::default(),
                );
                serialize(&mut encoder, city_model, self.formatting)?;
                encoder.finish()?;
            }
        }

        Ok(())
    }
}

impl GmlWriter<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let format = CitygmlFormat::from_path(path.as_ref()).ok_or_else(|| {
            InvalidFileExtension(
                path.as_ref()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or_default()
                    .to_string(),
            )
        })?;

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Self::new(file, format))
    }

    pub fn from_base_path_with_format(
        base_path: impl AsRef<Path>,
        format: CitygmlFormat,
    ) -> Result<Self, Error> {
        let mut path = base_path.as_ref().as_os_str().to_os_string();
        path.push(".");
        path.push(format.extension());

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Self::new(file, format))
    }
}

/// Controls whitespace formatting of serialized XML output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Formatting {
    /// All elements on a single line with no whitespace between them.
    Compact,
    /// Each element on its own line with no indentation.
    #[default]
    NewLine,
    /// Each element on its own line, indented by `size` repetitions of `char`.
    Indent { char: char, size: usize },
}
