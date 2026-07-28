use std::fs;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
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
    Clean { 
        path: PathBuf,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
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

fn find_junk(path: &Path) -> Vec<PathBuf> {
    let mut found = vec![];

    if !path.exists() {
        eprintln!("error: path does not exist: {}", path.display());
        return found;
    }
    if !path.is_dir() {
        eprintln!("error: not a directory: {}", path.display());
        return found;
    }

    let mut walker = WalkDir::new(path).follow_links(false).into_iter();

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
            found.push(entry.path().to_path_buf());
            walker.skip_current_dir();
        }
    }

    found
}

fn print_junk(paths: &[PathBuf]) {
    if paths.is_empty() {
        println!("No known junk folders found.");
        return;
    }

    for path in paths {
        println!(" {}", path.display());
    }

    println!("Found {} junk folder(s).", paths.len());
}

fn delete_junk(paths: &[PathBuf], dry_run: bool) {
    if paths.is_empty() {
        println!("Nothing to clean.");
        return;
    }

    let mut ok: usize = 0;

    for path in paths {
        if dry_run {
            println!(" [dry-run] would remove {}", path.display());
            ok += 1;
            continue;
        }

        match fs::remove_dir_all(path) {
            Ok(()) => {
                println!(" removed {}", path.display());
                ok += 1;
            }
            Err(err) => {
                eprintln!(" FAILED {}: {err}", path.display());
            }
        }
    }

    if dry_run {
        println!("Dry-run complete: {ok} folder(s) would be removed.");
        println!("Re-run without --dry-run to actually delete.");
    } else {
        println!("Done: removed {ok} of {} folder(s).", paths.len());
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            println!("Scanning under {} ...", path.display());
            let paths = find_junk(&path);
            print_junk(&paths);
        }
        Commands::Clean { path, dry_run } => {
            let mode = if dry_run { "DRY_RUN" } else { "LIVE" };
            println!("Cleaning under {} [{mode}] ...", path.display());

            let paths = find_junk(&path);
            print_junk(&paths);

            println!();

            delete_junk(&paths, dry_run);
        }
    }
}
