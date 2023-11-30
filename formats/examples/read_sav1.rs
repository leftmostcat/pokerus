use std::fs;

use clap::Parser;
use pokerus_data::Language;
use pokerus_formats::{gen1::SaveGen1, Pokemon as _, Save as _};

#[derive(Parser)]
struct Args {
    filename: String,
}

fn main() {
    let args = Args::parse();
    let sav = fs::read(args.filename).unwrap();

    let save = SaveGen1::try_from(&sav).unwrap();

    println!("{} ({})", save.trainer_name().unwrap(), save.trainer_id());
    println!("P{}", save.money());
    println!("Played {:?}", save.play_time());

    println!("\nPARTY");
    for mon in save.party() {
        println!(
            "{} ({}) ({}/{})",
            mon.nickname().unwrap(),
            mon.species().unwrap().name(Language::English),
            mon.current_hp(),
            mon.max_hp().unwrap()
        );
    }

    for (idx, pokemon_box) in save.boxes().iter().enumerate() {
        println!("\nBOX {}", idx + 1);
        for mon in pokemon_box.as_ref().iter() {
            println!(
                "{} ({}) ({}/{})",
                mon.nickname().unwrap(),
                mon.species().unwrap().name(Language::English),
                mon.current_hp(),
                mon.max_hp().unwrap()
            );
        }
    }
}
