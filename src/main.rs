use clap::{Parser, Subcommand};
use human_bytes::human_bytes;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};

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
    Scan {
        path: PathBuf,
    },
    Clean {
        path: PathBuf,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}

struct JunkEntry {
    path: PathBuf,
    size_bytes: u64,
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

fn find_junk_paths(path: &Path) -> Vec<PathBuf> {
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
            Ok(e) => e,
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
            // let path = entry.path().to_path_buf();
            // let size_bytes = dir_size(&path);
            // found.push(JunkEntry { path, size_bytes });
            found.push(entry.path().to_path_buf());
            walker.skip_current_dir();
        }
    }

    // found.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    found
}

fn dir_size(dir: &Path) -> u64 {
    WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

fn size_entries(paths: &[PathBuf]) -> Vec<JunkEntry> {
    if paths.is_empty() {
        return Vec::new();
    }

    let pb = ProgressBar::new(paths.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} sizing",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    let mut entries: Vec<JunkEntry> = paths.par_iter()
        .map(|path| {
            let entry = JunkEntry {
                path: path.clone(),
                size_bytes: dir_size(&path)
            };
            pb.inc(1);
            entry
        })
        .collect();

    pb.finish_and_clear();

    entries.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    entries
}

fn print_junk_report(paths: &[JunkEntry]) {
    if paths.is_empty() {
        println!("No known junk folders found.");
        return;
    }

    let total: u64 = paths.iter().map(|e| e.size_bytes).sum();

    for entry in paths {
        println!(
            " {:>10}   {}",
            human_bytes(entry.size_bytes as f64),
            entry.path.display()
        );
    }

    println!(
        "Found {} junk folder(s). Total: {}",
        paths.len(),
        human_bytes(total as f64)
    );
}

fn delete_junk(paths: &[JunkEntry], dry_run: bool) {
    if paths.is_empty() {
        println!("Nothing to clean.");
        return;
    }

    let mut ok: usize = 0;

    for entry in paths {
        if dry_run {
            println!(
                " [dry-run] would remove {:>10}  {}",
                human_bytes(entry.size_bytes as f64),
                entry.path.display()
            );
            ok += 1;
            continue;
        }

        match fs::remove_dir_all(&entry.path) {
            Ok(()) => {
                println!(
                    " removed {:>10}  {}",
                    human_bytes(entry.size_bytes as f64),
                    entry.path.display()
                );
                ok += 1;
            }
            Err(err) => {
                eprintln!(" FAILED {}: {err}", entry.path.display());
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
            let paths = find_junk_paths(&path);
            let entries = size_entries(&paths);
            print_junk_report(&entries);
        }
        Commands::Clean { path, dry_run } => {
            let mode = if dry_run { "DRY_RUN" } else { "LIVE" };
            println!("Cleaning under {} [{mode}] ...", path.display());

            let paths = find_junk_paths(&path);
            let entries = size_entries(&paths);
            print_junk_report(&entries);

            println!();

            delete_junk(&entries, dry_run);
        }
    }
}
