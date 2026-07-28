use clap::{Parser, Subcommand};
use std::path::PathBuf;
use walkdir::WalkDir;

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

fn scan_junk(path: &std::path::Path) {
    if !path.exists() {
        eprintln!("error: path does not exist: {}", path.display());
        return;
    }
    if !path.is_dir() {
        eprintln!("error: not a directory: {}", path.display());
        return;
    }

    let mut walker = WalkDir::new(path).follow_links(false).into_iter();

    let mut found = 0usize;

    while let Some(entry) = walker.next() {
        let entry = match entry {
            Ok(e) => {
                e
            },
            Err(err) => {
                eprintln!("warning: skipped entry: {err}");
                continue;
            }
        };

        if !entry.file_type().is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy();

        if JUNK_DIR_NAMES.contains(&name.as_ref()) {
            println!(" {}", entry.path().display());
            found += 1;
            walker.skip_current_dir();
        }
    }
    // let dirs = fs::read_dir(path);
    // let entries = match dirs {
    //     Ok(rd) => {
    //         rd
    //     }
    //     Err(failure) => {
    //         eprintln!("error: cannot read: {}: {failure}", path.display());
    //         return;
    //     }
    // };

    // for entry in entries {
    //     let entry = match entry {
    //         Ok(dir_entry) => {
    //             dir_entry
    //         },
    //         Err(failure) => {
    //             eprint!("warning: skipped entry: {}", failure);
    //             continue;
    //         }
    //     };

    //     let file_type = match entry.file_type() {
    //         Ok(file_type) => {
    //             file_type
    //         },
    //         Err(failure) => {
    //             eprintln!("warning:{}: {failure}", entry.path().display());
    //             continue;
    //         }
    //     };

    //     if !file_type.is_dir() {
    //         continue;
    //     }

    //     let name = entry.file_name();

    //     let name = name.to_string_lossy();

    //     if JUNK_DIR_NAMES.contains(&name.as_ref()) {
    //         found += 1;
    //         println!(" {}", entry.path().display());
    //     }
    // }

    if found == 0 {
        println!("No known junk folders under {}", path.display());
    } else {
        println!("Found {found} junk folder(s).");
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            println!("Scanning under {} ...", path.display());
            scan_junk(&path);
        }
        Commands::Clean { path } => {
            println!("clean: not implemented yet ({})", path.display());
        }
    }
    // println!("{cli:#?}");
}
