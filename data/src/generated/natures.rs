// GENERATED CONTENT; DO NOT EDIT. See the `protean` crate.
use crate::{NamesData, NatureData};
use common::Error;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Nature {
    Hardy,
    Lonely,
    Brave,
    Adamant,
    Naughty,
    Bold,
    Docile,
    Relaxed,
    Impish,
    Lax,
    Timid,
    Hasty,
    Serious,
    Jolly,
    Naive,
    Modest,
    Mild,
    Quiet,
    Bashful,
    Rash,
    Calm,
    Gentle,
    Sassy,
    Careful,
    Quirky,
}
impl Nature {
    pub(crate) fn data(&self) -> &'static NatureData {
        match self {
            Self::Hardy => &NatureData {
                names: NamesData {
                    names: [
                        "がんばりや",
                        "Hardy",
                        "Hardi",
                        "Ardita",
                        "Robust",
                        "Fuerte",
                        "노력",
                        "勤奋",
                        "勤奮",
                    ],
                },
            },
            Self::Lonely => &NatureData {
                names: NamesData {
                    names: [
                        "さみしがり",
                        "Lonely",
                        "Solo",
                        "Schiva",
                        "Solo",
                        "Huraña",
                        "외로움",
                        "怕寂寞",
                        "怕寂寞",
                    ],
                },
            },
            Self::Brave => &NatureData {
                names: NamesData {
                    names: [
                        "ゆうかん",
                        "Brave",
                        "Brave",
                        "Audace",
                        "Mutig",
                        "Audaz",
                        "용감",
                        "勇敢",
                        "勇敢",
                    ],
                },
            },
            Self::Adamant => &NatureData {
                names: NamesData {
                    names: [
                        "いじっぱり",
                        "Adamant",
                        "Rigide",
                        "Decisa",
                        "Hart",
                        "Firme",
                        "고집",
                        "固执",
                        "固執",
                    ],
                },
            },
            Self::Naughty => &NatureData {
                names: NamesData {
                    names: [
                        "やんちゃ",
                        "Naughty",
                        "Mauvais",
                        "Birbona",
                        "Frech",
                        "Pícara",
                        "개구쟁이",
                        "顽皮",
                        "頑皮",
                    ],
                },
            },
            Self::Bold => &NatureData {
                names: NamesData {
                    names: [
                        "ずぶとい",
                        "Bold",
                        "Assuré",
                        "Sicura",
                        "Kühn",
                        "Osada",
                        "대담",
                        "大胆",
                        "大膽",
                    ],
                },
            },
            Self::Docile => &NatureData {
                names: NamesData {
                    names: [
                        "すなお",
                        "Docile",
                        "Docile",
                        "Docile",
                        "Sanft",
                        "Dócil",
                        "온순",
                        "坦率",
                        "坦率",
                    ],
                },
            },
            Self::Relaxed => &NatureData {
                names: NamesData {
                    names: [
                        "のんき",
                        "Relaxed",
                        "Relax",
                        "Placida",
                        "Locker",
                        "Plácida",
                        "무사태평",
                        "悠闲",
                        "悠閒",
                    ],
                },
            },
            Self::Impish => &NatureData {
                names: NamesData {
                    names: [
                        "わんぱく",
                        "Impish",
                        "Malin",
                        "Scaltra",
                        "Pfiffig",
                        "Agitada",
                        "장난꾸러기",
                        "淘气",
                        "淘氣",
                    ],
                },
            },
            Self::Lax => &NatureData {
                names: NamesData {
                    names: [
                        "のうてんき",
                        "Lax",
                        "Lâche",
                        "Fiacca",
                        "Lasch",
                        "Floja",
                        "촐랑",
                        "乐天",
                        "樂天",
                    ],
                },
            },
            Self::Timid => &NatureData {
                names: NamesData {
                    names: [
                        "おくびょう",
                        "Timid",
                        "Timide",
                        "Timida",
                        "Scheu",
                        "Miedosa",
                        "겁쟁이",
                        "胆小",
                        "膽小",
                    ],
                },
            },
            Self::Hasty => &NatureData {
                names: NamesData {
                    names: [
                        "せっかち",
                        "Hasty",
                        "Pressé",
                        "Lesta",
                        "Hastig",
                        "Activa",
                        "성급",
                        "急躁",
                        "急躁",
                    ],
                },
            },
            Self::Serious => &NatureData {
                names: NamesData {
                    names: [
                        "まじめ",
                        "Serious",
                        "Sérieux",
                        "Seria",
                        "Ernst",
                        "Seria",
                        "성실",
                        "认真",
                        "認真",
                    ],
                },
            },
            Self::Jolly => &NatureData {
                names: NamesData {
                    names: [
                        "ようき",
                        "Jolly",
                        "Jovial",
                        "Allegra",
                        "Froh",
                        "Alegre",
                        "명랑",
                        "爽朗",
                        "爽朗",
                    ],
                },
            },
            Self::Naive => &NatureData {
                names: NamesData {
                    names: [
                        "むじゃき",
                        "Naive",
                        "Naïf",
                        "Ingenua",
                        "Naiv",
                        "Ingenua",
                        "천진난만",
                        "天真",
                        "天真",
                    ],
                },
            },
            Self::Modest => &NatureData {
                names: NamesData {
                    names: [
                        "ひかえめ",
                        "Modest",
                        "Modeste",
                        "Modesta",
                        "Mäßig",
                        "Modesta",
                        "조심",
                        "内敛",
                        "內斂",
                    ],
                },
            },
            Self::Mild => &NatureData {
                names: NamesData {
                    names: [
                        "おっとり",
                        "Mild",
                        "Doux",
                        "Mite",
                        "Mild",
                        "Afable",
                        "의젓",
                        "慢吞吞",
                        "慢吞吞",
                    ],
                },
            },
            Self::Quiet => &NatureData {
                names: NamesData {
                    names: [
                        "れいせい",
                        "Quiet",
                        "Discret",
                        "Quieta",
                        "Ruhig",
                        "Mansa",
                        "냉정",
                        "冷静",
                        "冷靜",
                    ],
                },
            },
            Self::Bashful => &NatureData {
                names: NamesData {
                    names: [
                        "てれや",
                        "Bashful",
                        "Pudique",
                        "Ritrosa",
                        "Zaghaft",
                        "Tímida",
                        "수줍음",
                        "害羞",
                        "害羞",
                    ],
                },
            },
            Self::Rash => &NatureData {
                names: NamesData {
                    names: [
                        "うっかりや",
                        "Rash",
                        "Foufou",
                        "Ardente",
                        "Hitzig",
                        "Alocada",
                        "덜렁",
                        "马虎",
                        "馬虎",
                    ],
                },
            },
            Self::Calm => &NatureData {
                names: NamesData {
                    names: [
                        "おだやか",
                        "Calm",
                        "Calme",
                        "Calma",
                        "Still",
                        "Serena",
                        "차분",
                        "温和",
                        "溫和",
                    ],
                },
            },
            Self::Gentle => &NatureData {
                names: NamesData {
                    names: [
                        "おとなしい",
                        "Gentle",
                        "Gentil",
                        "Gentile",
                        "Zart",
                        "Amable",
                        "얌전",
                        "温顺",
                        "溫順",
                    ],
                },
            },
            Self::Sassy => &NatureData {
                names: NamesData {
                    names: [
                        "なまいき",
                        "Sassy",
                        "Malpoli",
                        "Vivace",
                        "Forsch",
                        "Grosera",
                        "건방",
                        "自大",
                        "自大",
                    ],
                },
            },
            Self::Careful => &NatureData {
                names: NamesData {
                    names: [
                        "しんちょう",
                        "Careful",
                        "Prudent",
                        "Cauta",
                        "Sacht",
                        "Cauta",
                        "신중",
                        "慎重",
                        "慎重",
                    ],
                },
            },
            Self::Quirky => &NatureData {
                names: NamesData {
                    names: [
                        "きまぐれ",
                        "Quirky",
                        "Bizarre",
                        "Furba",
                        "Kauzig",
                        "Rara",
                        "변덕",
                        "浮躁",
                        "浮躁",
                    ],
                },
            },
        }
    }
}
impl TryFrom<u8> for Nature {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Error> {
        let value = match value {
            0 => Self::Hardy,
            1 => Self::Lonely,
            2 => Self::Brave,
            3 => Self::Adamant,
            4 => Self::Naughty,
            5 => Self::Bold,
            6 => Self::Docile,
            7 => Self::Relaxed,
            8 => Self::Impish,
            9 => Self::Lax,
            10 => Self::Timid,
            11 => Self::Hasty,
            12 => Self::Serious,
            13 => Self::Jolly,
            14 => Self::Naive,
            15 => Self::Modest,
            16 => Self::Mild,
            17 => Self::Quiet,
            18 => Self::Bashful,
            19 => Self::Rash,
            20 => Self::Calm,
            21 => Self::Gentle,
            22 => Self::Sassy,
            23 => Self::Careful,
            24 => Self::Quirky,
            _ => return Err(Error::invalid_argument()),
        };
        Ok(value)
    }
}
impl From<Nature> for u8 {
    fn from(value: Nature) -> Self {
        match value {
            Nature::Hardy => 0,
            Nature::Lonely => 1,
            Nature::Brave => 2,
            Nature::Adamant => 3,
            Nature::Naughty => 4,
            Nature::Bold => 5,
            Nature::Docile => 6,
            Nature::Relaxed => 7,
            Nature::Impish => 8,
            Nature::Lax => 9,
            Nature::Timid => 10,
            Nature::Hasty => 11,
            Nature::Serious => 12,
            Nature::Jolly => 13,
            Nature::Naive => 14,
            Nature::Modest => 15,
            Nature::Mild => 16,
            Nature::Quiet => 17,
            Nature::Bashful => 18,
            Nature::Rash => 19,
            Nature::Calm => 20,
            Nature::Gentle => 21,
            Nature::Sassy => 22,
            Nature::Careful => 23,
            Nature::Quirky => 24,
        }
    }
}
