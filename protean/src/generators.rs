use std::{fs, io, path::PathBuf};

use proc_macro2::TokenStream;

mod experience;
mod items;
mod moves;
mod natures;
mod species;
mod types;

pub use experience::generate_experience_functions;
pub use items::generate_items_data;
pub use moves::generate_moves_data;
pub use natures::generate_natures_data;
pub use species::generate_species_data;
pub use types::generate_types_data;

fn write_generated_file(path: &PathBuf, tokens: TokenStream) -> io::Result<()> {
    let commented = format!(
        "// GENERATED CONTENT; DO NOT EDIT. See the `protean` crate.\n{}",
        tokens
    );

    fs::write(path, commented)
}
