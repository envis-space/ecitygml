use clap::{Parser, Subcommand, ValueEnum, ValueHint};
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

    /// Apply a rigid-body transform (translation + rotation) to a CityGML file
    Transform {
        /// Input file
        #[clap(short, long, value_hint = ValueHint::FilePath)]
        input: PathBuf,

        /// Output file (defaults to <input_stem>_transformed.<ext>)
        #[clap(short, long, value_hint = ValueHint::FilePath)]
        output: Option<PathBuf>,

        /// Translation along X axis
        #[clap(long, default_value = "0.0")]
        tx: f64,

        /// Translation along Y axis
        #[clap(long, default_value = "0.0")]
        ty: f64,

        /// Translation along Z axis
        #[clap(long, default_value = "0.0")]
        tz: f64,

        /// Rotation around X axis (roll) in degrees
        #[clap(long, default_value = "0.0")]
        rx: f64,

        /// Rotation around Y axis (pitch) in degrees
        #[clap(long, default_value = "0.0")]
        ry: f64,

        /// Rotation around Z axis (yaw) in degrees
        #[clap(long, default_value = "0.0")]
        rz: f64,

        /// Output formatting style
        #[clap(long, default_value = "newline")]
        formatting: FormattingArg,

        /// Indentation size in spaces (only used with --formatting indent)
        #[clap(long, default_value = "2")]
        indent_size: usize,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FormattingArg {
    /// All elements on a single line with no whitespace
    Compact,
    /// Each element on its own line, no indentation
    Newline,
    /// Each element on its own line, indented with spaces
    Indent,
}
