use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs;

#[derive(Parser, Debug)]
#[command(
    name = "dev-clean",
    version,
    about = "Scan or Delete Developer Junk Folders"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan { path: PathBuf },
    Clean { path: PathBuf },
}

const JUNK_DIR_NAMES: &[&str] = &[
    "dist",
    "build",
    "target",
    "node_modules",
    ".angular",
    ".next",
    ".cache",
    "__pycache__",
];

fn scan_top_level(path: &std::path::Path) {
    let dirs = fs::read_dir(path);
    let entries = match dirs {
        Ok(rd) => {
            rd
        }
        Err(failure) => {
            eprintln!("error: cannot read: {}: {failure}", path.display());
            return;
        }
    };

    let mut found = 0usize;

    for entry in entries {
        let entry = match entry {
            Ok(dir_entry) => {
                dir_entry
            },
            Err(failure) => {
                eprint!("warning: skipped entry: {}", failure);
                continue;
            }
        };

        let file_type = match entry.file_type() {
            Ok(file_type) => {
                file_type
            },
            Err(failure) => {
                eprintln!("warning:{}: {failure}", entry.path().display());
                continue;
            }
        };

        if !file_type.is_dir() {
            continue;
        }

        let name = entry.file_name();

        let name = name.to_string_lossy();

        if JUNK_DIR_NAMES.contains(&name.as_ref()) {
            found += 1;
            println!(" {}", entry.path().display());
        }
    }

    if found == 0 {
        println!("No known junk folders at top level of {}", path.display());
    } else {
        println!("Found {found} junk folder(s).");
    }
}

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            println!("Scanning top level of {} ...", path.display());
            scan_top_level(&path);
        }
        Commands::Clean { path } => {
            println!("clean: not implemented yet ({})", path.display());
        }
    }
    // println!("{cli:#?}");
}
