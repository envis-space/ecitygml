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
        Commands::Statistics { file_path } => {
            commands::statistics::run(file_path.canonicalize()?)?;
        }
        Commands::ListObjects { file_path } => {
            commands::list_objects::run(file_path.canonicalize()?)?;
        }
        Commands::Validate {
            file_path,
            output_directory_path,
        } => {
            commands::validate::run(file_path.canonicalize()?, output_directory_path)?;
        }
    };

    Ok(())
}
