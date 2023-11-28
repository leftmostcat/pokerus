use common::Error;

use super::{
    constants::{CollectionType, Edition},
    PokemonGen1,
};

pub(super) struct PokemonListIter<'a> {
    data: &'a [u8],
    edition: Edition,
    collection_type: CollectionType,
    position: usize,
}

impl<'a> PokemonListIter<'a> {
    pub(crate) fn try_from_values(
        data: &'a [u8],
        edition: Edition,
        collection_type: CollectionType,
        expected_count: usize,
    ) -> Result<Self, Error> {
        let count = data[0] as usize;
        if count != expected_count {
            log::error!(target: "gen1", "list count {count} exceeded did not match expected {expected_count}");
            return Err(Error::invalid_data_value());
        }

        if data.len() == calculate_size_of_collection(count, edition, collection_type) {
            Ok(Self {
                data,
                edition,
                collection_type,
                position: 0,
            })
        } else {
            Err(Error::invalid_data_value())
        }
    }
}

impl<'a> Iterator for PokemonListIter<'a> {
    type Item = PokemonGen1<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self.len();
        if self.position >= count {
            return None;
        }

        let header_size = calculate_header_size(count);
        let data_size = self.collection_type.pokemon_data_size();
        let name_size = self.edition.name_length();

        let all_mons_data_size = data_size * count;

        let mon_data_offset = header_size + self.position * data_size;
        let mon_data = &self.data[mon_data_offset..mon_data_offset + data_size];

        let original_trainer_name_offset =
            header_size + all_mons_data_size + self.position * name_size;
        let original_trainer_name =
            &self.data[original_trainer_name_offset..original_trainer_name_offset + name_size];

        let nickname_offset =
            header_size + all_mons_data_size + count * name_size + self.position * name_size;
        let nickname = &self.data[nickname_offset..nickname_offset + name_size];

        self.position += 1;

        Some(PokemonGen1::from_data(
            mon_data,
            original_trainer_name,
            nickname,
            self.edition,
            None,
        ))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a> ExactSizeIterator for PokemonListIter<'a> {
    fn len(&self) -> usize {
        self.data[0] as usize
    }
}

pub(crate) const fn calculate_size_of_collection(
    length: usize,
    edition: Edition,
    collection_type: CollectionType,
) -> usize {
    // Data for each Pokémon is stored non-contiguously: the structure first
    // contains 0x2f bytes of data per Pokémon, followed by one original trainer
    // name per Pokémon, then one nickname per Pokémon.
    calculate_header_size(length)
        + length * (collection_type.pokemon_data_size() + 2 * edition.name_length())
}

const fn calculate_header_size(length: usize) -> usize {
    // The header consists of a single byte `n` for the count of contained
    // Pokémon, a list of `n` species indices, and a single `0xFF` byte to
    // indicate the end of the list.
    1 + (length + 1)
}
