use std::{io, path::Path};

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use super::write_generated_file;

pub fn generate_experience_functions(out_dir: &Path) -> io::Result<()> {
    let erratic_matches = functions_for_rate("erratic", calculate_min_experience_for_level_erratic);
    let fast_matches = functions_for_rate("fast", calculate_min_experience_for_level_fast);
    let medium_fast_matches = functions_for_rate(
        "medium_fast",
        calculate_min_experience_for_level_medium_fast,
    );
    let medium_slow_matches = functions_for_rate(
        "medium_slow",
        calculate_min_experience_for_level_medium_slow,
    );
    let slow_matches = functions_for_rate("slow", calculate_min_experience_for_level_slow);
    let fluctuating_matches = functions_for_rate(
        "fluctuating",
        calculate_min_experience_for_level_fluctuating,
    );

    let tokens = quote!(
        use common::Error;

        #erratic_matches
        #fast_matches
        #medium_fast_matches
        #medium_slow_matches
        #slow_matches
        #fluctuating_matches
    );

    write_generated_file(&out_dir.join("experience.rs"), tokens)
}

fn functions_for_rate(rate: &str, calculator: fn(u32) -> u32) -> TokenStream {
    let values: Vec<_> = (1..=100)
        .map(|level| {
            (
                {
                    let value = calculator(level);
                    Literal::u32_unsuffixed(value)
                },
                if level < 100 {
                    {
                        let value = calculator(level + 1) - 1;
                        Literal::u32_unsuffixed(value)
                    }
                } else {
                    Literal::u32_unsuffixed(0)
                },
            )
        })
        .collect();

    let value_tokens = values
        .iter()
        .enumerate()
        .map(|(level, values)| (Literal::usize_unsuffixed(level + 1), values));

    let min_experience_tokens: TokenStream = value_tokens
        .clone()
        .map(|(level, (min, _))| quote!(#level => #min,))
        .collect();

    let level_tokens: TokenStream = value_tokens
        .take(99)
        .map(|(level, (min, max))| quote!(#min..=#max => #level,))
        .collect();

    let level_100_value = {
        let value = calculator(100);
        Literal::u32_unsuffixed(value)
    };

    let min_experience_name = format_ident!("min_experience_for_level_{rate}");
    let calculate_name = format_ident!("calculate_level_{rate}");

    quote!(
        pub(crate) const fn #min_experience_name(level: u8) -> Result<u32, Error> {
            let experience = match level {
                #min_experience_tokens
                _ => return Err(Error::invalid_argument()),
            };

            Ok(experience)
        }

        pub(crate) const fn #calculate_name(experience: u32) -> Result<u8, Error> {
            let level = match experience {
                #level_tokens
                #level_100_value => 100,
                _ => return Err(Error::invalid_argument()),
            };

            Ok(level)
        }
    )
}

fn calculate_min_experience_for_level_erratic(level: u32) -> u32 {
    match level {
        1 => 0,
        2..=49 => (level.pow(3) * (100 - level)) / 50,
        50..=67 => (level.pow(3) * (150 - level)) / 100,
        68..=97 => (level.pow(3) * ((1911 - 10 * level) / 3)) / 500,
        98..=100 => (level.pow(3) * (160 - level)) / 100,
        _ => panic!("Invalid level {level}"),
    }
}

fn calculate_min_experience_for_level_fast(level: u32) -> u32 {
    match level {
        1 => 0,
        2..=100 => (4 * level.pow(3)) / 5,
        _ => panic!("Invalid level {level}"),
    }
}

fn calculate_min_experience_for_level_medium_fast(level: u32) -> u32 {
    match level {
        1 => 0,
        2..=100 => level.pow(3),
        _ => panic!("Invalid level {level}"),
    }
}

fn calculate_min_experience_for_level_medium_slow(level: u32) -> u32 {
    match level {
        1 => 0,
        2..=100 => ((6 * level.pow(3)) / 5)
            .wrapping_sub(15 * level.pow(2))
            .wrapping_add(100 * level - 140),
        _ => panic!("Invalid level {level}"),
    }
}

fn calculate_min_experience_for_level_slow(level: u32) -> u32 {
    match level {
        1 => 0,
        2..=100 => (5 * level.pow(3)) / 4,
        _ => panic!("Invalid level {level}"),
    }
}

fn calculate_min_experience_for_level_fluctuating(level: u32) -> u32 {
    match level {
        1 => 0,
        2..=14 => (level.pow(3) * (((level + 1) / 3) + 24)) / 50,
        15..=35 => (level.pow(3) * (level + 14)) / 50,
        36..=100 => (level.pow(3) * ((level / 2) + 32)) / 50,
        _ => panic!("Invalid level {level}"),
    }
}
