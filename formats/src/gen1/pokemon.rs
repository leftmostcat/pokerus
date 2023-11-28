use core::str;

use alloc::vec::Vec;
use common::Error;
use pokerus_data::{calculate_level, GameSet, Language, Move, NonVolatileStatus, Species};

use crate::{utils::lazy_string::LazyString, HasInternalSpc, HasSpc, Pokemon, PokemonMove};

use super::{
    constants::{CollectionType, Edition},
    list::PokemonListIter,
    utils::{read_u16, read_u24},
};

pub struct PokemonGen1<'a> {
    data: &'a [u8],
    original_trainer_name: LazyString<'a, Error>,
    nickname: LazyString<'a, Error>,
    edition: Edition,
    language: Option<Language>,
}

impl<'a> PokemonGen1<'a> {
    /// Sets a language of origin for the Pokémon data. This is used in
    /// determining whether the Pokémon is nicknamed.
    ///
    /// NOTE: Due to limitations in the gen1 data structure, language of origin
    /// is not persisted.
    pub fn hint_language_of_origin(&mut self, language: Language) -> Result<(), Error> {
        if !self.edition.can_encode_language(language) {
            return Err(Error::invalid_argument());
        }

        self.language = Some(language);

        Ok(())
    }

    const SPECIES_IDX_OFFSET: usize = 0x00;
    const CURRENT_HP_OFFSET: usize = 0x01;
    // A copy of the level is stored at byte 0x03.
    const STATUS_CONDITION_OFFSET: usize = 0x04;
    // Copies of species primary and secondary type are stored at bytes 0x05 and
    // 0x06, respectively.
    // A copy of the catch rate is stored at byte 0x07. It is reused as held
    // item in Gen 2.
    const MOVE_INDICES_OFFSET: usize = 0x08;
    const ORIGINAL_TRAINER_ID_OFFSET: usize = 0x0c;
    const EXPERIENCE_OFFSET: usize = 0x0e;
    const EV_OFFSET: usize = 0x11;
    const DV_OFFSET: usize = 0x1b;
    const PP_VALUES_OFFSET: usize = 0x1d;
    // When a Pokémon is in the trainer's party:
    // A second copy of level is stored at byte 0x21.
    // Copies of the calculated values for HP, Attack, Defense, Speed, and
    // Special are stored in that order as 16-bit values starting at byte 0x22.

    pub(crate) fn from_data(
        data: &'a [u8],
        original_trainer_name: &'a [u8],
        nickname: &'a [u8],
        edition: Edition,
        language_hint: Option<Language>,
    ) -> Self {
        let language = match edition {
            Edition::Japanese => Some(Language::Japanese),
            Edition::International => language_hint,
        };

        Self {
            data,
            original_trainer_name: LazyString::from(original_trainer_name),
            nickname: LazyString::from(nickname),
            language,
            edition,
        }
    }

    fn calculate_stat(&self, base: u8, dv: u8, ev: u16) -> Result<u16, Error> {
        let ev_contrib = Self::gen1_sqrt(ev) / 4;

        let base = base as u16;
        let dv = dv as u16;
        Ok(((((base + dv) * 2 + ev_contrib) * self.level()? as u16) / 100) + 5)
    }

    /// Gets the integer square root of a value, with all results falling in the
    /// inclusive range [1..255].
    ///
    /// The values returned by this function are not always mathematically
    /// accurate, but they match the values produced by stat calculation in
    /// Gen 1 Pokémon games.
    fn gen1_sqrt(value: u16) -> u16 {
        for sqrt in 1..=254 {
            if sqrt * sqrt >= value {
                return sqrt;
            }
        }

        // In the games, this value is stored in an 8-bit register. It will
        // always return 255 as the value for values greater than 64516.
        255
    }

    fn _default_name(&self) -> Vec<u8> {
        todo!()
    }
}

impl<'a> Pokemon for PokemonGen1<'a> {
    fn species(&self) -> Result<Species, Error> {
        // Only base forms exist in Gen 1, so we
        static FORM_ID: u8 = 0;

        let id = self.data[Self::SPECIES_IDX_OFFSET].into();
        Species::try_from_species_and_form_id(id, FORM_ID, GameSet::RedGreenBlue).map_err(|_| {
            log::error!(target: "pk1", "unable to match species ID {id}");

            Error::invalid_data_value()
        })
    }

    fn nickname(&self) -> Result<&str, Error> {
        self.nickname
            .get_or_try_decode(self.edition.codec())
            .map_err(|&err| err)
    }

    fn is_nicknamed(&self) -> bool {
        todo!()
    }

    fn experience(&self) -> u32 {
        read_u24(self.data, Self::EXPERIENCE_OFFSET)
    }

    fn level(&self) -> Result<u8, Error> {
        calculate_level(
            self.experience(),
            self.species()?
                .experience_growth_rate(GameSet::RedGreenBlue)?,
        )
    }

    fn original_trainer_id(&self) -> u16 {
        read_u16(self.data, Self::ORIGINAL_TRAINER_ID_OFFSET)
    }

    fn original_trainer(&self) -> Result<&str, Error> {
        self.original_trainer_name
            .get_or_try_decode(self.edition.codec())
            .map_err(|&err| err)
    }

    fn move_values(&self, idx: usize) -> Result<Option<PokemonMove>, Error> {
        if idx >= 4 {
            return Err(Error::invalid_argument());
        }

        let value = self.data[Self::MOVE_INDICES_OFFSET + idx];
        if value == 0 {
            if idx == 0 {
                return Err(Error::invalid_data_value());
            } else {
                return Ok(None);
            }
        }

        let (current_pp, pp_ups) = {
            // Current PP and PP Up count are stored as bits 0..6 and 6..8
            // respectively of the same value.
            let pp_value = self.data[Self::PP_VALUES_OFFSET + idx];

            (pp_value & 0b0011_1111, pp_value >> 6)
        };

        Ok(Some(PokemonMove {
            value: Move::try_from(value as u16)?,
            current_pp,
            pp_ups,
        }))
    }

    fn status_condition(&self) -> Result<Option<NonVolatileStatus>, Error> {
        let value = match self.data[Self::STATUS_CONDITION_OFFSET] {
            0x00 => return Ok(None),
            0x04 => NonVolatileStatus::Asleep,
            0x08 => NonVolatileStatus::Poisoned,
            0x10 => NonVolatileStatus::Burned,
            0x20 => NonVolatileStatus::Frozen,
            0x40 => NonVolatileStatus::Paralyzed,
            _ => return Err(Error::invalid_data_value()),
        };

        Ok(Some(value))
    }

    fn iv_hp(&self) -> u8 {
        // HP does not have a separate stored DV value, instead being made up of
        // the least-significant bits of each of the other DVs.
        let dv_values = &self.data[Self::DV_OFFSET..Self::DV_OFFSET + 2];
        ((dv_values[0] & 0x10) >> 1)
            | ((dv_values[0] & 0x01) << 2)
            | ((dv_values[1] & 0x10) >> 3)
            | (dv_values[1] & 0x01)
    }

    fn iv_atk(&self) -> u8 {
        self.data[Self::DV_OFFSET] >> 4
    }

    fn iv_def(&self) -> u8 {
        self.data[Self::DV_OFFSET] & 0x0f
    }

    fn iv_spe(&self) -> u8 {
        self.data[Self::DV_OFFSET + 1] >> 4
    }

    fn ev_hp(&self) -> u16 {
        read_u16(self.data, Self::EV_OFFSET)
    }

    fn ev_atk(&self) -> u16 {
        read_u16(self.data, Self::EV_OFFSET + 2)
    }

    fn ev_def(&self) -> u16 {
        read_u16(self.data, Self::EV_OFFSET + 4)
    }

    fn ev_spe(&self) -> u16 {
        read_u16(self.data, Self::EV_OFFSET + 6)
    }

    fn current_hp(&self) -> u16 {
        read_u16(self.data, Self::CURRENT_HP_OFFSET)
    }

    fn max_hp(&self) -> Result<u16, Error> {
        // Base stats don't change from RGB to Y, so we can use RGB without loss
        // of generality.
        let base_hp = self.species()?.base_hit_points(GameSet::RedGreenBlue)?;
        Ok(self.calculate_stat(base_hp, self.iv_hp(), self.ev_hp())? + self.level()? as u16 + 5)
    }

    fn atk(&self) -> Result<u16, Error> {
        // Base stats don't change from RGB to Y, so we can use RGB without loss
        // of generality.
        let base_atk = self.species()?.base_attack(GameSet::RedGreenBlue)?;
        self.calculate_stat(base_atk, self.iv_atk(), self.ev_atk())
    }

    fn def(&self) -> Result<u16, Error> {
        // Base stats don't change from RGB to Y, so we can use RGB without loss
        // of generality.
        let base_def = self.species()?.base_defense(GameSet::RedGreenBlue)?;
        self.calculate_stat(base_def, self.iv_def(), self.ev_def())
    }

    fn spe(&self) -> Result<u16, Error> {
        // Base stats don't change from RGB to Y, so we can use RGB without loss
        // of generality.
        let base_spe = self.species()?.base_speed(GameSet::RedGreenBlue)?;
        self.calculate_stat(base_spe, self.iv_spe(), self.ev_spe())
    }
}

impl<'a> HasInternalSpc for PokemonGen1<'a> {
    fn iv_spc(&self) -> u8 {
        self.data[Self::DV_OFFSET + 1] & 0x0f
    }

    fn ev_spc(&self) -> u16 {
        read_u16(self.data, Self::EV_OFFSET + 8)
    }
}

impl<'a> HasSpc for PokemonGen1<'a> {
    fn spc(&self) -> Result<u16, Error> {
        // Base stats don't change from Red/Blue to Y, so we can use RB without
        // loss of generality.
        let base_spc = self.species()?.base_special(GameSet::RedGreenBlue)?;
        self.calculate_stat(base_spc, self.iv_spc(), self.ev_spc())
    }
}

impl<'a> TryFrom<&'a [u8]> for PokemonGen1<'a> {
    type Error = Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let edition = if value.len() == 0x45 {
            Edition::International
        } else if value.len() == 0x3b {
            Edition::Japanese
        } else {
            return Err(Error::format_mismatch());
        };

        let pokemon = PokemonListIter::try_from_values(value, edition, CollectionType::Party, 1)?
            .next()
            .unwrap();

        Ok(pokemon)
    }
}
