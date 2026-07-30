use clap::{Parser, Subcommand};
use std::path::{PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "dev-clean",
    version,
    about = "Scan or Delete Developer Junk Folders"
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