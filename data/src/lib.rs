#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod generated;

mod experience;
pub use experience::*;

mod game_sets;
pub use game_sets::*;

mod gender;
pub use gender::*;

mod items;
pub use items::*;

mod languages;
pub use languages::*;

mod moves;
pub use moves::*;

mod natures;
pub use natures::*;

mod pokerus;
pub use pokerus::*;

mod species;
pub use species::*;

mod status;
pub use status::*;

mod types;
pub use types::*;

pub(crate) struct NamesData {
    pub(crate) names: [&'static str; 9],
}

impl NamesData {
    pub(crate) fn get(&self, language: Language) -> &'static str {
        match language {
            Language::Japanese => self.names[0],
            Language::English => self.names[1],
            Language::French => self.names[2],
            Language::Italian => self.names[3],
            Language::German => self.names[4],
            Language::Spanish => self.names[5],
            Language::Korean => self.names[6],
            Language::ChineseSimplified => self.names[7],
            Language::ChineseTraditional => self.names[8],
        }
    }
}
