use std::fs;

use clap::Parser;
use pokerus_formats::{gen1::SaveGen1, Pokemon as _, Save as _};

#[derive(Parser)]
struct Args {
    filename: String,
}

fn main() {
    let args = Args::parse();
    let sav = fs::read(args.filename).unwrap();

    let save = SaveGen1::try_from_slice(&sav).unwrap();

    println!("{} ({})", save.trainer_name().unwrap(), save.trainer_id());
    println!("P{}", save.money());
    println!("Played {:?}", save.play_time());

    println!("\nPARTY");
    for mon in save.party() {
        println!(
            "{} ({:?}) ({}/{})",
            mon.nickname().unwrap(),
            mon.species().unwrap(),
            mon.current_hp(),
            mon.max_hp().unwrap()
        );
    }
}
