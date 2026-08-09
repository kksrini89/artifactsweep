use clap::{Parser, Subcommand};
use std::path::{PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "sweep",
    version,
    about = "Scan or Clean developer generated artifact folders"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Scan {
        path: PathBuf,
    },
    Clean {
        path: PathBuf,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}