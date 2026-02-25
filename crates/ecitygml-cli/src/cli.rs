use clap::ValueHint;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compute some statistics about the dataset
    Statistics {
        /// Input directory
        #[clap(short, long, value_hint = ValueHint::FilePath)]
        file_path: PathBuf,
    },

    /// List all objects with their metadata
    ListObjects {
        /// Input file
        #[clap(short, long, value_hint = ValueHint::FilePath)]
        file_path: PathBuf,
    },

    /// Validate the dataset
    Validate {
        /// Input directory
        #[clap(short, long, value_hint = ValueHint::FilePath)]
        file_path: PathBuf,

        /// Output directory
        #[clap(short, long, value_hint = ValueHint::DirPath)]
        output_directory_path: PathBuf,
    },
}
