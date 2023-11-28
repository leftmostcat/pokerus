use pokerus_data::Language;

use super::utils::Gen1Codec;

#[derive(Clone, Copy)]
pub(crate) enum Edition {
    Japanese,
    International,
}

impl Edition {
    pub(crate) const fn can_encode_language(&self, language: Language) -> bool {
        match self {
            Edition::Japanese => matches!(language, Language::Japanese),
            Edition::International => matches!(
                language,
                Language::English
                    | Language::French
                    | Language::Italian
                    | Language::German
                    | Language::Spanish
            ),
        }
    }

    pub(crate) fn codec(&self) -> &'static Gen1Codec {
        match self {
            Edition::Japanese => &JAPANESE_CODEC,
            Edition::International => &INTERNATIONAL_CODEC,
        }
    }

    pub const fn name_length(&self) -> usize {
        match self {
            Edition::Japanese => JPN_NAME_LENGTH,
            Edition::International => INTL_NAME_LENGTH,
        }
    }

    pub const fn trainer_id_offset(&self) -> usize {
        match self {
            Edition::Japanese => JPN_TID_OFFSET,
            Edition::International => INTL_TID_OFFSET,
        }
    }

    pub const fn rival_name_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x25f1,
            Edition::International => 0x25f6,
        }
    }

    pub const fn money_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x25ee,
            Edition::International => 0x25f3,
        }
    }

    pub const fn play_time_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x2ca0,
            Edition::International => 0x2ced,
        }
    }

    pub const fn party_offset(&self) -> usize {
        match self {
            Edition::Japanese => JPN_PARTY_OFFSET,
            Edition::International => INTL_PARTY_OFFSET,
        }
    }

    pub const fn box_count(&self) -> usize {
        match self {
            Edition::Japanese => JPN_BOX_COUNT,
            Edition::International => INTL_BOX_COUNT,
        }
    }

    pub const fn count_per_box(&self) -> usize {
        match self {
            Edition::Japanese => JPN_COUNT_PER_BOX,
            Edition::International => INTL_COUNT_PER_BOX,
        }
    }

    pub const fn current_box_idx_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x2842,
            Edition::International => 0x284c,
        }
    }

    pub const fn current_box_offset(&self) -> usize {
        match self {
            Edition::Japanese => JPN_OPEN_BOX_OFFSET,
            Edition::International => INTL_OPEN_BOX_OFFSET,
        }
    }

    pub const fn _bag_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x25c4,
            Edition::International => 0x25c9,
        }
    }
}

pub(crate) enum CollectionType {
    Party,
    Box,
}

impl CollectionType {
    pub const fn pokemon_data_size(&self) -> usize {
        match self {
            CollectionType::Party => PARTY_POKEMON_SIZE,
            CollectionType::Box => BOX_POKEMON_SIZE,
        }
    }
}

pub(super) const PARTY_POKEMON_SIZE: usize = 0x2c;
pub(super) const BOX_POKEMON_SIZE: usize = 0x21;

pub(super) const JPN_PARTY_OFFSET: usize = 0x2ed5;
pub(super) const INTL_PARTY_OFFSET: usize = 0x2f2c;

pub(super) const JPN_BOX_COUNT: usize = 8;
pub(super) const INTL_BOX_COUNT: usize = 12;

pub(super) const JPN_COUNT_PER_BOX: usize = 30;
pub(super) const INTL_COUNT_PER_BOX: usize = 20;

pub(super) const JPN_OPEN_BOX_OFFSET: usize = 0x302d;
pub(super) const INTL_OPEN_BOX_OFFSET: usize = 0x30c0;

pub(super) const JPN_NAME_LENGTH: usize = 0x06;
pub(super) const INTL_NAME_LENGTH: usize = 0x0b;

pub(super) const JPN_TID_OFFSET: usize = 0x25fb;
pub(super) const INTL_TID_OFFSET: usize = 0x2605;

static JAPANESE_CODEC: Gen1Codec = Gen1Codec {
    edition: Edition::Japanese,
};

static INTERNATIONAL_CODEC: Gen1Codec = Gen1Codec {
    edition: Edition::International,
};
