mod cli;
mod commands;
mod error;

use anyhow::Result;

use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Statistics { input } => {
            commands::statistics::run(input.canonicalize()?)?;
        }
        Commands::ListObjects { file_path } => {
            commands::list_objects::run(file_path.canonicalize()?)?;
        }
        Commands::Transform {
            input,
            output,
            tx,
            ty,
            tz,
            rx,
            ry,
            rz,
            formatting,
            indent_size,
        } => {
            let input = input.canonicalize()?;
            let output = match output {
                Some(p) => p.clone(),
                None => {
                    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
                    let ext = input.extension().unwrap_or_default().to_string_lossy();
                    input.with_file_name(format!("{stem}_transformed.{ext}"))
                }
            };
            commands::transform::run(
                &input,
                &output,
                *tx,
                *ty,
                *tz,
                *rx,
                *ry,
                *rz,
                *formatting,
                *indent_size,
            )?;
        }
    };

    Ok(())
}
