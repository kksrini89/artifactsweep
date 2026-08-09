mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use artifactsweep_core::{
    delete_junk, find_junk_paths, 
    print_junk_report, size_entries,
};

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
