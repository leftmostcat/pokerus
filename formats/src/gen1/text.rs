use alloc::{string::String, vec::Vec};
use common::Error;

use crate::utils::lazy_string::StringCodec;

use super::constants::Edition;

/// `Gen1Codec` provides encoding and decoding methods for the several character
/// encodings used in Red, Green, Blue, and Yellow.
pub(crate) struct Gen1Codec {
    pub(crate) edition: Edition,
}

impl StringCodec for Gen1Codec {
    type Error = Error;

    fn try_decode(&self, value: &[u8]) -> Result<String, Self::Error> {
        let mapper = match self.edition {
            Edition::Japanese => map_japanese_character_to_utf8,
            Edition::International => map_international_character_to_utf8,
        };

        value
            .iter()
            .take_while(|&&character| character != 0x50)
            .map(mapper)
            .collect()
    }

    fn try_encode(&self, _value: &str) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }
}

/// Determines whether a string contains any of the characters which can only be
/// input in the German editions of the games.
pub(crate) fn gen1_string_contains_german_characters(string: &[u8]) -> bool {
    // `0xc0` through `0xc5` can't be input through normal means except in the
    // German version of the games, where they represent `ÄÖÜäöü` respectively.
    string
        .iter()
        .any(|character| (0xc0..=0xc5).contains(character))
}

/// Maps characters from the Japanese editions of Red, Green, Blue, and Yellow
/// to UTF-8 equivalents.
pub(crate) fn map_japanese_character_to_utf8(_character: &u8) -> Result<char, Error> {
    todo!()
}

/// Maps characters from international editions of Red, Blue, and Yellow to
/// UTF-8 equivalents.
///
/// Though different encodings are used for English, French/German, Italian, and
/// Spanish, the limited sets of characters which appear in Pokémon names or
/// which players can input allows us to use a single map function for all five.
///
/// Private Use Area characters are used for characters which have no Unicode
/// equivalent.
pub(crate) fn map_international_character_to_utf8(character: &u8) -> Result<char, Error> {
    let character = match character {
        // Control code which prints "TRAINER" in the game's language. Mapped to
        // a Private Use Area character.
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

        0xe0 => '\u{2019}', // RIGHT SINGLE QUOTATION MARK

        // "PK" glyph. Mapped to a Private Use Area character following the
        // convention used in the 3DS core series.
        0xe1 => '\u{e0a7}',

        // "MN" glyph. Mapped to a Private Use Area character following the
        // convention used in the 3DS core series.
        0xe2 => '\u{e0a8}',

        0xe3 => '-',

        0xe6 => '?',
        0xe7 => '!',
        0xe8 => '.',

        0xef => '♂',

        0xf1 => '\u{00d7}', // MULTIPLICATION SIGN
        0xf2 => '.',
        0xf3 => '/',
        0xf4 => ',',
        0xf5 => '♀',

        character => {
            log::error!(target: "gen1", "intl string contained unexpected value {character:#04x}");
            return Err(Error::invalid_data_value());
        }
    };

    Ok(character)
}
