use crate::error::Error;
use crate::gml::read_impl::decode;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::error::Error::InvalidFileExtension;
use crate::format::CitygmlFormat;
use crate::gml::validate_impl::validate_from_reader;
use ecitygml_core::model::core::CityModel;
use std::path::Path;

/// `CitygmlReader` reads CityGML datasets.
///
#[derive(Debug, Clone)]
pub struct GmlReader<R: Read> {
    reader: R,
    format: CitygmlFormat,
    rebuild_object_bounds: bool,
}

impl<R: Read> GmlReader<R> {
    /// Create a new [`GmlReader`] from an existing `Reader`.
    pub fn new(reader: R, format: CitygmlFormat) -> Self {
        Self {
            reader,
            format,
            rebuild_object_bounds: true,
        }
    }

    fn read(self) -> Result<Vec<u8>, Error> {
        let mut content = Vec::new();

        match self.format {
            CitygmlFormat::Gml => {
                BufReader::new(self.reader).read_to_end(&mut content)?;
            }
            CitygmlFormat::GmlZst => {
                zstd::Decoder::new(BufReader::new(self.reader))?.read_to_end(&mut content)?;
            }
            CitygmlFormat::GmlGz => {
                flate2::read::GzDecoder::new(BufReader::new(self.reader))
                    .read_to_end(&mut content)?;
            }
        }

        Ok(content)
    }

    pub fn validate(self) -> Result<crate::gml::validate::report::Report, Error> {
        validate_from_reader(self.read()?)
    }

    pub fn with_rebuild_object_bounds(mut self, rebuild_object_bounds: bool) -> Self {
        self.rebuild_object_bounds = rebuild_object_bounds;
        self
    }

    pub fn finish(self) -> Result<CityModel, Error> {
        let rebuild_object_bounds = self.rebuild_object_bounds;
        let mut city_model = decode(self.read()?)?;

        if rebuild_object_bounds {
            city_model.refresh_bounded_by_recursive();
        }

        Ok(city_model)
    }
}

impl GmlReader<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let format = CitygmlFormat::from_path(path.as_ref()).ok_or(InvalidFileExtension(
            path.as_ref()
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        ))?;

        let file = std::fs::File::open(path.as_ref())?;

        Ok(Self::new(file, format))
    }
}
