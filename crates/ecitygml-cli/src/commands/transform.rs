use crate::cli::FormattingArg;
use crate::error::Error;
use ecitygml::io::{Formatting, GmlWriter};
use nalgebra::{Isometry3, Translation3, UnitQuaternion};
use std::path::Path;
use std::time::Instant;
use tracing::{info, warn};

pub fn run(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    tx: f64,
    ty: f64,
    tz: f64,
    rx_deg: f64,
    ry_deg: f64,
    rz_deg: f64,
    formatting_arg: FormattingArg,
    indent_size: usize,
) -> Result<(), Error> {
    info!("Reading: {}", input.as_ref().display());
    let read_start = Instant::now();
    let mut city_model = ecitygml::io::GmlReader::from_path(input.as_ref())?.finish()?;
    info!("Read in {:.3?}", read_start.elapsed());

    let is_identity =
        tx == 0.0 && ty == 0.0 && tz == 0.0 && rx_deg == 0.0 && ry_deg == 0.0 && rz_deg == 0.0;

    if is_identity {
        warn!("Transform is identity — no transformation will be applied");
    } else {
        let translation = Translation3::new(tx, ty, tz);
        let rotation = UnitQuaternion::from_euler_angles(
            rx_deg.to_radians(),
            ry_deg.to_radians(),
            rz_deg.to_radians(),
        );
        let isometry = Isometry3::from_parts(translation, rotation);
        city_model.apply_transform(&isometry);
    }

    let formatting = match formatting_arg {
        FormattingArg::Compact => Formatting::Compact,
        FormattingArg::Newline => Formatting::NewLine,
        FormattingArg::Indent => Formatting::Indent {
            char: ' ',
            size: indent_size,
        },
    };

    info!("Writing: {}", output.as_ref().display());
    let write_start = Instant::now();
    GmlWriter::from_path(output.as_ref())?
        .with_formatting(formatting)
        .finish(city_model)?;
    info!("Written in {:.3?}", write_start.elapsed());

    Ok(())
}
