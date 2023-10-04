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

        // The header consists of a single byte `n` for the number of contained
        // Pokémon, a list of `n` species indices, and a single `0xFF` byte to
        // indicate the end of the list.
        let header_size = 1 + (count + 1);

        let data_size = collection_type.pokemon_data_size();
        let name_length = edition.name_length();

        if data.len() == header_size + count * (data_size + 2 * name_length) {
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

        let header_size = get_header_size_from_count(count);
        let data_size = self.collection_type.pokemon_data_size();
        let name_size = self.edition.name_length();

        let all_mons_data_size = data_size * count;

        // Data for each Pokémon is stored non-contiguously: the structure
        // first contains 0x2f bytes of data per Pokémon, followed by one
        // original trainer name per Pokémon, then one nickname per Pokémon.
        // Pokémon data is stored non-contiguously:
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

fn get_header_size_from_count(count: usize) -> usize {
    1 + (count + 1)
}
