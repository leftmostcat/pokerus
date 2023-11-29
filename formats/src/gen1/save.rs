use core::time::Duration;

use alloc::vec::Vec;
use common::Error;
use pokerus_data::Language;

use crate::{
    utils::{data_reader::DataReader as _, lazy_string::LazyString},
    Save,
};

use super::{
    constants::{CollectionType, Edition},
    list::PokemonListIter,
    text::gen1_string_contains_german_characters,
    PokemonGen1,
};

pub struct SaveGen1<'a> {
    data: &'a [u8],
    edition: Edition,
    _language: Option<Language>,

    trainer_name: LazyString<'a, Error>,

    party: Vec<PokemonGen1<'a>>,
}

impl<'a> SaveGen1<'a> {
    pub fn try_from_slice(data: &'a [u8]) -> Result<Self, Error> {
        if data.len() != 0x8000 && data.len() != 0x802c {
            return Err(Error::format_mismatch());
        }

        let edition = if are_collections_valid_for_edition(data, Edition::International) {
            Edition::International
        } else if are_collections_valid_for_edition(data, Edition::Japanese) {
            Edition::Japanese
        } else {
            return Err(Error::invalid_data_value());
        };

        let trainer_name_data = &data[0x2598..0x2598 + edition.name_length()];
        let trainer_name = LazyString::from(trainer_name_data);

        let rival_name_data =
            &data[edition.rival_name_offset()..edition.rival_name_offset() + edition.name_length()];
        let language = match edition {
            Edition::Japanese => Some(Language::Japanese),
            Edition::International => {
                if gen1_string_contains_german_characters(trainer_name_data)
                    || gen1_string_contains_german_characters(rival_name_data)
                {
                    Some(Language::German)
                } else {
                    None
                }
            }
        };

        let party = PokemonListIter::try_from_values(
            &data[edition.party_offset()..],
            edition,
            CollectionType::Party,
            6,
        )?
        .collect();

        Ok(Self {
            data,
            edition,
            _language: language,

            trainer_name,

            party,
        })
    }
}

impl<'a> Save<'a, PokemonGen1<'a>> for SaveGen1<'a> {
    fn trainer_id(&self) -> u32 {
        self.data.read_u16_be(self.edition.trainer_id_offset()) as u32
    }

    fn trainer_name(&self) -> Result<&str, Error> {
        self.trainer_name
            .get_or_try_decode(self.edition.codec())
            .map_err(|&err| err)
    }

    fn money(&self) -> u32 {
        let money_bcd = self.data.read_u24_be(self.edition.money_offset());

        (0..6).fold(0, |money, place| {
            let shift = 4 * place;
            let mask = 0xf << shift;

            let digit = (money_bcd & mask) >> shift;
            money + digit * 10u32.pow(place)
        })
    }

    fn play_time(&self) -> Duration {
        let played_hours = self.data[self.edition.play_time_offset()] as u64;
        let played_minutes = self.data[self.edition.play_time_offset() + 1] as u64;
        let played_seconds = self.data[self.edition.play_time_offset() + 2] as u64;

        let total_played_seconds = played_seconds + 60 * (played_minutes + 60 * played_hours);

        Duration::from_secs(total_played_seconds)
    }

    fn party(&self) -> &[PokemonGen1<'a>] {
        &self.party
    }

    fn box_count(&self) -> usize {
        self.edition.box_count()
    }

    fn current_box_idx(&self) -> usize {
        self.data[self.edition.current_box_idx_offset()] as usize & 0x7f
    }
}

fn are_collections_valid_for_edition(data: &[u8], edition: Edition) -> bool {
    let party_data = &data[edition.party_offset()..];
    let party_is_valid = get_collection_is_valid(party_data, 6);

    let open_box_data = &data[edition.current_box_offset()..];
    let is_open_box_valid = get_collection_is_valid(open_box_data, edition.count_per_box());

    party_is_valid && is_open_box_valid
}

fn get_collection_is_valid(data: &[u8], max_entries: usize) -> bool {
    data[0] <= max_entries as u8 && data[1 + data[0] as usize] == 0xff
}
