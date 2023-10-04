use std::{
    path::{Path, PathBuf},
    process,
};

use clap::{Parser, Subcommand};
use protean::*;

fn main() {
    let args = Args::parse();

    match args.subcommand {
        Command::Codegen => {
            let xtask_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

            let source_dir = xtask_dir.join("../sources/");
            let dest_dir = xtask_dir.join("../data/src/generated/");

            codegen(&source_dir, &dest_dir)
        }
    }
}

fn codegen(source_dir: &Path, dest_dir: &Path) {
    generate_experience_functions(dest_dir).unwrap();
    generate_items_data(source_dir, dest_dir).unwrap();
    generate_moves_data(source_dir, dest_dir).unwrap();
    generate_natures_data(source_dir, dest_dir).unwrap();
    generate_species_data(source_dir, dest_dir).unwrap();
    generate_types_data(source_dir, dest_dir).unwrap();

    process::Command::new(env!("CARGO"))
        .arg("fmt")
        .output()
        .expect("Unable to run `cargo fmt`");
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    subcommand: Command,
}

#[derive(Subcommand)]
enum Command {
    Codegen,
}
