use pokerus_data::Language;

use super::text::Gen1Codec;

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

    pub(crate) const fn name_length(&self) -> usize {
        match self {
            Edition::Japanese => JPN_NAME_LENGTH,
            Edition::International => INTL_NAME_LENGTH,
        }
    }

    pub(crate) const fn trainer_id_offset(&self) -> usize {
        match self {
            Edition::Japanese => JPN_TID_OFFSET,
            Edition::International => INTL_TID_OFFSET,
        }
    }

    pub(crate) const fn rival_name_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x25f1,
            Edition::International => 0x25f6,
        }
    }

    pub(crate) const fn money_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x25ee,
            Edition::International => 0x25f3,
        }
    }

    pub(crate) const fn play_time_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x2ca0,
            Edition::International => 0x2ced,
        }
    }

    pub(crate) const fn party_offset(&self) -> usize {
        match self {
            Edition::Japanese => JPN_PARTY_OFFSET,
            Edition::International => INTL_PARTY_OFFSET,
        }
    }

    pub(crate) const fn box_count(&self) -> usize {
        match self {
            Edition::Japanese => JPN_BOX_COUNT,
            Edition::International => INTL_BOX_COUNT,
        }
    }

    pub(crate) const fn count_per_box(&self) -> usize {
        match self {
            Edition::Japanese => JPN_COUNT_PER_BOX,
            Edition::International => INTL_COUNT_PER_BOX,
        }
    }

    pub(crate) const fn current_box_idx_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x2842,
            Edition::International => 0x284c,
        }
    }

    pub(crate) const fn current_box_offset(&self) -> usize {
        match self {
            Edition::Japanese => JPN_OPEN_BOX_OFFSET,
            Edition::International => INTL_OPEN_BOX_OFFSET,
        }
    }

    pub(crate) const fn _bag_offset(&self) -> usize {
        match self {
            Edition::Japanese => 0x25c4,
            Edition::International => 0x25c9,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum CollectionType {
    Party,
    _Box,
}

impl CollectionType {
    pub(crate) const fn pokemon_data_size(&self) -> usize {
        match self {
            CollectionType::Party => PARTY_POKEMON_SIZE,
            CollectionType::_Box => BOX_POKEMON_SIZE,
        }
    }
}

const PARTY_POKEMON_SIZE: usize = 0x2c;
const BOX_POKEMON_SIZE: usize = 0x21;

const JPN_PARTY_OFFSET: usize = 0x2ed5;
const INTL_PARTY_OFFSET: usize = 0x2f2c;

const JPN_BOX_COUNT: usize = 8;
const INTL_BOX_COUNT: usize = 12;

const JPN_COUNT_PER_BOX: usize = 30;
const INTL_COUNT_PER_BOX: usize = 20;

const JPN_OPEN_BOX_OFFSET: usize = 0x302d;
const INTL_OPEN_BOX_OFFSET: usize = 0x30c0;

const JPN_NAME_LENGTH: usize = 0x06;
const INTL_NAME_LENGTH: usize = 0x0b;

const JPN_TID_OFFSET: usize = 0x25fb;
const INTL_TID_OFFSET: usize = 0x2605;

static JAPANESE_CODEC: Gen1Codec = Gen1Codec {
    edition: Edition::Japanese,
};

static INTERNATIONAL_CODEC: Gen1Codec = Gen1Codec {
    edition: Edition::International,
};
