use core::time::Duration;

use alloc::vec::Vec;
use common::Error;
use pokerus_data::Language;

use crate::{
    utils::{data_reader::DataReader as _, lazy_string::LazyString},
    ListBox, Save,
};

use super::{
    constants::{CollectionType, Edition},
    list::{calculate_size_of_collection, PokemonListIter},
    text::gen1_string_contains_german_characters,
    PokemonGen1,
};

pub struct SaveGen1<'a> {
    data: &'a [u8],
    edition: Edition,
    language: Option<Language>,

    trainer_name: LazyString<'a, Error>,

    party: Vec<PokemonGen1<'a>>,
    boxes: Vec<ListBox<PokemonGen1<'a>>>,
}

impl<'a> SaveGen1<'a> {
    pub fn hint_save_language(&mut self, language: Language) -> Result<(), Error> {
        if !self.edition.can_encode_language(language) {
            return Err(Error::invalid_argument());
        }

        self.language = Some(language);
        self.party
            .iter_mut()
            .chain(
                self.boxes
                    .iter_mut()
                    .flat_map(|list_box| list_box.as_mut().iter_mut()),
            )
            .for_each(|pokemon| pokemon.hint_language_of_origin(language).unwrap());

        Ok(())
    }

    /// Gets the friendship of the player's partner Pikachu.
    ///
    /// This value is only meaningful for Pokémon Yellow saves. Since these
    /// saves are otherwise identical to other versions, there is no trivial way
    /// to verify this.
    pub fn pikachu_friendship(&self) -> u8 {
        self.data[self.edition.pikachu_friendship_offset()]
    }
}

impl<'a> Save<'a, PokemonGen1<'a>> for SaveGen1<'a> {
    type PokemonBox = ListBox<PokemonGen1<'a>>;

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

    fn boxes(&self) -> &[Self::PokemonBox] {
        &self.boxes
    }
}

impl<'a> TryFrom<&'a [u8]> for SaveGen1<'a> {
    type Error = Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 0x8000 && value.len() != 0x802c {
            return Err(Error::format_mismatch());
        }

        let edition = if are_collections_valid_for_edition(value, Edition::International) {
            Edition::International
        } else if are_collections_valid_for_edition(value, Edition::Japanese) {
            Edition::Japanese
        } else {
            return Err(Error::invalid_data_value());
        };

        let checksum = calculate_checksum(&value[0x2598..edition.checksum_offset()]);
        if checksum != value[edition.checksum_offset()] {
            return Err(Error::invalid_data_value());
        }

        let trainer_name_data = &value[0x2598..0x2598 + edition.name_length()];
        let trainer_name = LazyString::from(trainer_name_data);

        let rival_name_data = &value
            [edition.rival_name_offset()..edition.rival_name_offset() + edition.name_length()];
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
            &value[edition.party_offset()..],
            edition,
            CollectionType::Party,
            6,
        )?
        .collect();

        let box_size =
            calculate_size_of_collection(edition.count_per_box(), edition, CollectionType::Box);
        let boxes = (0..edition.box_count())
            .map(|idx| {
                let offset = if idx < edition.box_count() / 2 {
                    0x4000 + idx * box_size
                } else {
                    let idx = idx - edition.box_count() / 2;
                    0x6000 + idx * box_size
                };

                let list_box = PokemonListIter::try_from_values(
                    &value[offset..],
                    edition,
                    CollectionType::Box,
                    edition.count_per_box(),
                )?
                .collect();

                Ok(list_box)
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            data: value,
            edition,
            language,

            trainer_name,

            party,
            boxes,
        })
    }
}

impl<'a> TryFrom<&'a Vec<u8>> for SaveGen1<'a> {
    type Error = Error;

    fn try_from(value: &'a Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value as &[u8])
    }
}

fn are_collections_valid_for_edition(data: &[u8], edition: Edition) -> bool {
    let party_data = &data[edition.party_offset()..];
    let party_is_valid = is_valid_collection(party_data, 6);

    let open_box_data = &data[edition.current_box_offset()..];
    let is_open_box_valid = is_valid_collection(open_box_data, edition.count_per_box());

    party_is_valid && is_open_box_valid
}

fn is_valid_collection(data: &[u8], max_entries: usize) -> bool {
    data[0] <= max_entries as u8 && data[1 + data[0] as usize] == 0xff
}

fn calculate_checksum(data: &[u8]) -> u8 {
    data.iter()
        .fold(0, |sum, &value| u8::wrapping_add(sum, value))
        ^ 0xff
}
