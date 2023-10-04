use alloc::string::String;
use common::Error;
use pokerus_data::Language;

use crate::utils::lazy_data::LazyData;

use super::constants::{CollectionType, Edition};

pub(crate) type ExoticString<'a> = LazyData<&'a [u8], String, Error>;

pub(crate) const fn calculate_size_of_collection(
    length: usize,
    edition: Edition,
    collection_type: CollectionType,
) -> usize {
    2 + length * (1 + collection_type.pokemon_data_size() + 2 * edition.name_length())
}

pub(crate) fn gen1_string_contains_german_characters(string: &[u8]) -> bool {
    // `0xc0` through `0xc5` can't be input through normal means except in the
    // German version of the games, where they represent `ÄÖÜäöü` respectively.
    string
        .iter()
        .any(|character| (0xc0..=0xc5).contains(character))
}

pub(crate) fn decode_gen1_string(data: &[u8], language: Option<Language>) -> Result<String, Error> {
    let mapper = match language {
        Some(Language::Japanese) => map_japanese_character_to_utf8,
        Some(Language::English)
        | Some(Language::French)
        | Some(Language::Italian)
        | Some(Language::German)
        | Some(Language::Spanish)
        | None => map_international_character_to_utf8,
        _ => return Err(Error::invalid_argument()),
    };

    data.iter()
        .take_while(|&&character| character != 0x50)
        .map(mapper)
        .collect()
}

pub(crate) fn map_japanese_character_to_utf8(_character: &u8) -> Result<char, Error> {
    todo!()
}

pub(crate) fn map_international_character_to_utf8(character: &u8) -> Result<char, Error> {
    let character = match character {
        // Control code which prints "TRAINER" in the game's language.
        0x5d => '\u{10005d}',

        0x7f => ' ',

        0x80 => 'A',
        0x81 => 'B',
        0x82 => 'C',
        0x83 => 'D',
        0x84 => 'E',
        0x85 => 'F',
        0x86 => 'G',
        0x87 => 'H',
        0x88 => 'I',
        0x89 => 'J',
        0x8a => 'K',
        0x8b => 'L',
        0x8c => 'M',
        0x8d => 'N',
        0x8e => 'O',
        0x8f => 'P',

        0x90 => 'Q',
        0x91 => 'R',
        0x92 => 'S',
        0x93 => 'T',
        0x94 => 'U',
        0x95 => 'V',
        0x96 => 'W',
        0x97 => 'X',
        0x98 => 'Y',
        0x99 => 'Z',
        0x9a => '(',
        0x9b => ')',
        0x9c => ':',
        0x9d => ';',
        0x9e => '[',
        0x9f => ']',

        0xa0 => 'a',
        0xa1 => 'b',
        0xa2 => 'c',
        0xa3 => 'd',
        0xa4 => 'e',
        0xa5 => 'f',
        0xa6 => 'g',
        0xa7 => 'h',
        0xa8 => 'i',
        0xa9 => 'j',
        0xaa => 'k',
        0xab => 'l',
        0xac => 'm',
        0xad => 'n',
        0xae => 'o',
        0xaf => 'p',

        0xb0 => 'q',
        0xb1 => 'r',
        0xb2 => 's',
        0xb3 => 't',
        0xb4 => 'u',
        0xb5 => 'v',
        0xb6 => 'w',
        0xb7 => 'x',
        0xb8 => 'y',
        0xb9 => 'z',

        // Used in the Spanish name of the trainer in Olivine City in GSC who
        // will trade his Voltorb for Krabby.
        0xbf => 'Á',

        // May be input for trainer names or nicknames in the German versions of
        // Gen 1/2 games.
        0xc0 => 'Ä',
        0xc1 => 'Ö',
        0xc2 => 'Ü',
        0xc3 => 'ä',
        0xc4 => 'ö',
        0xc5 => 'ü',

        // Used in the Spanish name of the trainer in Cianwood City in GSC who
        // will lend you his Shuckle.
        0xc9 => 'Í',

        0xe0 => '\u{2019}',

        // "PK" glyph.
        0xe1 => '\u{1000e1}',

        // "MN" glyph.
        0xe2 => '\u{1000e2}',

        0xe3 => '-',

        0xe6 => '?',
        0xe7 => '!',
        0xe8 => '.',

        0xef => '♂',

        0xf1 => '\u{00d7}',
        0xf2 => '.',
        0xf3 => '/',
        0xf4 => ',',
        0xf5 => '♀',

        character => {
            log::error!(target: "gen1", "intl string contained unexpected value {character}");
            return Err(Error::invalid_data_value());
        }
    };

    Ok(character)
}

pub(crate) fn read_u16(data: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([data[offset], data[offset + 1]])
}

pub(crate) fn read_u24(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]])
}
