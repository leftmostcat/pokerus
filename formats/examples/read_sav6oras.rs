use std::{env, fs};

use pokerus_formats::{gen6::Sav6ORAS, HasGender as _, Pokemon as _, Save as _};

fn main() {
    let filename = env::args().nth(1).unwrap();
    let sav = fs::read(filename).unwrap();

    let save = Sav6ORAS::from_bytes(&sav).unwrap();
    for mon in save.party() {
        println!(
            "species is {:?}\ngender is {:?}",
            mon.species().unwrap(),
            mon.gender().unwrap()
        );
    }
}
