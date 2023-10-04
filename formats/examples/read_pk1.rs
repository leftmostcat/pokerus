use std::fs;

use clap::Parser;
use common::Error;
use pokerus_data::{GameSet, Language, NonVolatileStatus};
use pokerus_formats::{gen1::PokemonGen1, HasInternalSpc as _, HasSpc as _, Pokemon as _};

#[derive(Parser)]
struct Args {
    filename: String,
}

fn main() {
    let args = Args::parse();

    let data = fs::read(args.filename).expect("Unable to read Pk1 file");
    let mon = PokemonGen1::try_from(data.as_ref()).expect("Unable to read Pokémon");

    // Nickname is fallible due to needing to be decoded from original encoding.
    // Lazily decoded, but only needs to be decoded once.
    println!("Nickname: {}", mon.nickname().unwrap_or("<INVALID>"));

    // Species is fallible since not all values map to a species. Glitch Pokémon
    // are considered invalid.
    println!(
        "Species: {}",
        mon.species()
            .map(|species| species.name(Language::English))
            .unwrap_or("<INVALID>")
    );

    // Level is fallible since experience may exceed the value for level 100.
    // API is available from the `data` crate to get that cap value.
    let level = match mon.level() {
        Ok(level) => format!("{level}"),
        Err(_) => String::from("<INVALID>"),
    };
    println!(
        "Lv. {level} (Exp.: {experience})",
        experience = mon.experience()
    );

    // Original trainer name behaves the same as nickname above.
    println!(
        "OT: {name} ({id})",
        name = mon.original_trainer().unwrap_or("<INVALID>"),
        id = mon.original_trainer_id()
    );

    let type_name = mon
        .species()
        .and_then(|species| {
            // Getting game-specific species data is fallible because not every
            // species is present in every game.
            let primary = species.primary_type(GameSet::RedGreenBlue)?;
            let secondary = species.secondary_type(GameSet::RedGreenBlue)?;

            let type_name = if primary == secondary {
                String::from(primary.name(Language::English))
            } else {
                format!(
                    "{primary} / {secondary}",
                    primary = primary.name(Language::English),
                    secondary = secondary.name(Language::English)
                )
            };

            Ok(type_name)
        })
        .unwrap_or(String::from("<INVALID>"));
    println!("Type: {type_name}");

    // Maximum HP is fallible due to reliance on species being valid, since it
    // requires knowing the Pokémon's species.
    let max = match mon.max_hp() {
        Ok(max) => format!("{max}"),
        Err(_) => String::from("<INVALID>"),
    };
    println!(
        "HP: {current}/{max} (DV: {dv}, EV: {ev})",
        current = mon.current_hp(),
        dv = mon.iv_hp(),
        ev = mon.ev_hp()
    );

    // Status is fallible since not all values match to a status condition and
    // some values may match to more than one, which shouldn't be possible.
    match mon.status_condition() {
        Ok(status) => {
            if let Some(status) = status {
                let status = match status {
                    NonVolatileStatus::Asleep => "SLP",
                    NonVolatileStatus::Poisoned => "PSN",

                    // Badly poisoned is volatile in Gen 1.
                    NonVolatileStatus::BadlyPoisoned => unreachable!(),

                    NonVolatileStatus::Burned => "BRN",
                    NonVolatileStatus::Frozen => "FRZ",
                    NonVolatileStatus::Paralyzed => "PAR",
                };

                println!("Status: {status}");
            }
        }
        Err(_) => println!("Status: <INVALID>"),
    };

    // All other calculated stats are fallible for the same reason as max HP.
    print_stat("Atk", mon.iv_atk(), mon.ev_atk(), mon.atk());
    print_stat("Def", mon.iv_def(), mon.ev_def(), mon.def());
    print_stat("Spc", mon.iv_spc(), mon.ev_spc(), mon.spc());
    print_stat("Spe", mon.iv_spe(), mon.ev_spe(), mon.spe());

    for idx in 0..4 {
        // Getting the move in a position may fail either the function is called
        // with an invalid index or because the data does not contain a valid
        // move value.
        match mon.move_values(idx) {
            Ok(move_values) => {
                // There may be no move in a slot, and in such cases, no values
                // are returned at all.
                match move_values {
                    Some(move_values) => {
                        // Base PP is per-game, and so may fail if a move is not
                        // in the specified games.
                        let max = match move_values.value.base_pp(GameSet::RedGreenBlue) {
                            Ok(base_pp) => format!(
                                "{max} ({base_pp} + {pp_ups})",
                                max = base_pp + move_values.pp_ups,
                                pp_ups = move_values.pp_ups
                            ),
                            Err(_) => String::from("<INVALID>"),
                        };

                        println!(
                            "\n{name}: {current} PP/{max}",
                            name = move_values.value.name(Language::English),
                            current = move_values.current_pp
                        );

                        // Other game-specific move values are fallible for the
                        // same reason as base PP.
                        let power = match move_values.value.power(GameSet::RedGreenBlue) {
                            Ok(power) => format!("{power}"),
                            Err(_) => String::from("<INVALID>"),
                        };
                        let accuracy = match move_values.value.accuracy(GameSet::RedGreenBlue) {
                            Ok(accuracy) => format!("{accuracy}"),
                            Err(_) => String::from("<INVALID>"),
                        };
                        let type_name = move_values
                            .value
                            .move_type(GameSet::RedGreenBlue)
                            .map(|move_type| move_type.name(Language::English))
                            .unwrap_or("<INVALID>");
                        println!("Power: {power} | Accuracy: {accuracy} | Type: {type_name}");
                    }
                    None => break,
                }
            }
            Err(_) => println!("\n<INVALID MOVE>"),
        };
    }
}

fn print_stat(stat: &str, dv: u8, ev: u16, calculated: Result<u16, Error>) {
    let calculated = match calculated {
        Ok(calculated) => format!("{calculated}"),
        Err(_) => String::from("<INVALID>"),
    };
    println!("{stat}: {calculated} (DV: {dv}, EV: {ev})");
}
