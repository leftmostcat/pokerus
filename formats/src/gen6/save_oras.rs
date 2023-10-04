use core::time::Duration;

use alloc::vec::Vec;
use common::Error;
use pokerus_data::{GameSet, Gender, Nature, NonVolatileStatus, Species};

use crate::{
    crypto::decrypt_gen_6, HasGender, HasInternalSpaSpd, HasNature, HasSpaSpd, Pokemon,
    PokemonMove, Save,
};

const SAVE_FILE_SIZE: usize = 0x76000;
const PARTY_OFFSET: usize = 0x14200;
const PK6_SIZE_IN_PARTY: usize = 0x104;

pub struct Sav6ORAS {
    _data: Vec<u8>,
    party: Vec<Pk6>,
}

impl Sav6ORAS {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Error> {
        if data.len() != SAVE_FILE_SIZE {
            return Err(Error::format_mismatch());
        }

        let mut party = Vec::with_capacity(6);
        for index in 0..6 {
            let data_start = PARTY_OFFSET + PK6_SIZE_IN_PARTY * index;
            let pokemon_data = &data[data_start..data_start + PK6_SIZE_IN_PARTY];
            if pokemon_data[8..10] == [0, 0] {
                // The species data is unset, indicating that there is no
                // Pokémon in this slot.
                break;
            }

            party.push(Pk6::from_bytes(pokemon_data)?);
        }

        if party.is_empty() {
            // There should be at least one Pokémon in the party at all times,
            // but the first slot had no valid species data.
            return Err(Error::invalid_data_value());
        }

        Ok(Self {
            _data: Vec::from(data),
            party,
        })
    }
}

impl Save<'_, Pk6> for Sav6ORAS {
    fn trainer_id(&self) -> u32 {
        todo!()
    }

    fn trainer_name(&self) -> Result<&str, Error> {
        todo!()
    }

    fn money(&self) -> u32 {
        todo!()
    }

    fn play_time(&self) -> Duration {
        todo!()
    }

    fn party(&self) -> &[Pk6] {
        &self.party
    }

    fn box_count(&self) -> usize {
        todo!()
    }

    fn current_box_idx(&self) -> usize {
        todo!()
    }
}

pub struct Pk6 {
    data: Vec<u8>,
}

impl Pk6 {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 0xe8 && data.len() != PK6_SIZE_IN_PARTY {
            return Err(Error::format_mismatch());
        }

        let data = Self::decrypt(data);
        Ok(Self { data })
    }

    fn decrypt(data: &[u8]) -> Vec<u8> {
        let mut data = Vec::from(data);

        if data[0x58] == 0 && data[0xc8] == 0 {
            // These two bytes are unused and always zero in unencrypted Pokémon
            // data, so we use this as a heuristic for determining whether
            // decryption is needed.
            return data;
        }

        decrypt_gen_6(&mut data);

        data
    }
}

impl Pokemon for Pk6 {
    fn species(&self) -> Result<Species, Error> {
        let species_id = u16::from_le_bytes([self.data[8], self.data[9]]);
        let form_id = self.data[0x1d] >> 3;
        Species::try_from_species_and_form_id(species_id, form_id, GameSet::OmegaRubyAlphaSapphire)
    }

    fn nickname(&self) -> Result<&str, Error> {
        todo!()
    }

    fn is_nicknamed(&self) -> bool {
        todo!()
    }

    fn original_trainer_id(&self) -> u16 {
        todo!()
    }

    fn original_trainer(&self) -> Result<&str, Error> {
        todo!()
    }

    fn experience(&self) -> u32 {
        u32::from_le_bytes([
            self.data[0x10],
            self.data[0x11],
            self.data[0x12],
            self.data[0x13],
        ])
    }

    fn level(&self) -> Result<u8, Error> {
        todo!()
    }

    fn move_values(&self, _idx: usize) -> Result<Option<PokemonMove>, Error> {
        todo!()
    }

    fn status_condition(&self) -> Result<Option<NonVolatileStatus>, Error> {
        todo!()
    }

    fn iv_hp(&self) -> u8 {
        todo!()
    }

    fn iv_atk(&self) -> u8 {
        todo!()
    }

    fn iv_def(&self) -> u8 {
        todo!()
    }

    fn iv_spe(&self) -> u8 {
        todo!()
    }

    fn ev_hp(&self) -> u16 {
        todo!()
    }

    fn ev_atk(&self) -> u16 {
        todo!()
    }

    fn ev_def(&self) -> u16 {
        todo!()
    }

    fn ev_spe(&self) -> u16 {
        todo!()
    }

    fn current_hp(&self) -> u16 {
        todo!()
    }

    fn max_hp(&self) -> Result<u16, Error> {
        todo!()
    }

    fn atk(&self) -> Result<u16, Error> {
        todo!()
    }

    fn def(&self) -> Result<u16, Error> {
        todo!()
    }

    fn spe(&self) -> Result<u16, Error> {
        todo!()
    }
}

impl HasGender for Pk6 {
    fn gender(&self) -> Result<Gender, Error> {
        let value = match (self.data[0x1d] >> 1) & 3 {
            0 => Gender::Male,
            1 => Gender::Female,
            2 => Gender::Unknown,

            _ => return Err(Error::invalid_data_value()),
        };

        Ok(value)
    }
}

impl HasSpaSpd for Pk6 {
    fn spa(&self) -> Result<u16, Error> {
        todo!()
    }

    fn spd(&self) -> Result<u16, Error> {
        todo!()
    }
}

impl HasInternalSpaSpd for Pk6 {
    fn iv_spa(&self) -> u8 {
        todo!()
    }

    fn iv_spd(&self) -> u8 {
        todo!()
    }

    fn ev_spa(&self) -> u16 {
        todo!()
    }

    fn ev_spd(&self) -> u16 {
        todo!()
    }
}

impl HasNature for Pk6 {
    fn nature(&self) -> Result<Nature, Error> {
        todo!()
    }
}
