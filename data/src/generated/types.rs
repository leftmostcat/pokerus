// GENERATED CONTENT; DO NOT EDIT. See the `protean` crate.
use crate::{NamesData, TypeData};
use common::Error;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    QuestionMark,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}
impl Type {
    pub(crate) fn data(&self) -> &'static TypeData {
        match self {
            Self::Normal => &TypeData {
                names: NamesData {
                    names: [
                        "ノーマル",
                        "Normal",
                        "Normal",
                        "Normale",
                        "Normal",
                        "Normal",
                        "노말",
                        "一般",
                        "一般",
                    ],
                },
            },
            Self::Fighting => &TypeData {
                names: NamesData {
                    names: [
                        "かくとう",
                        "Fighting",
                        "Combat",
                        "Lotta",
                        "Kampf",
                        "Lucha",
                        "격투",
                        "格斗",
                        "格鬥",
                    ],
                },
            },
            Self::Flying => &TypeData {
                names: NamesData {
                    names: [
                        "ひこう",
                        "Flying",
                        "Vol",
                        "Volante",
                        "Flug",
                        "Volador",
                        "비행",
                        "飞行",
                        "飛行",
                    ],
                },
            },
            Self::Poison => &TypeData {
                names: NamesData {
                    names: [
                        "どく", "Poison", "Poison", "Veleno", "Gift", "Veneno", "독", "毒", "毒",
                    ],
                },
            },
            Self::Ground => &TypeData {
                names: NamesData {
                    names: [
                        "じめん",
                        "Ground",
                        "Sol",
                        "Terra",
                        "Boden",
                        "Tierra",
                        "땅",
                        "地面",
                        "地面",
                    ],
                },
            },
            Self::Rock => &TypeData {
                names: NamesData {
                    names: [
                        "いわ", "Rock", "Roche", "Roccia", "Gestein", "Roca", "바위", "岩石",
                        "岩石",
                    ],
                },
            },
            Self::Bug => &TypeData {
                names: NamesData {
                    names: [
                        "むし",
                        "Bug",
                        "Insecte",
                        "Coleottero",
                        "Käfer",
                        "Bicho",
                        "벌레",
                        "虫",
                        "蟲",
                    ],
                },
            },
            Self::Ghost => &TypeData {
                names: NamesData {
                    names: [
                        "ゴースト",
                        "Ghost",
                        "Spectre",
                        "Spettro",
                        "Geist",
                        "Fantasma",
                        "고스트",
                        "幽灵",
                        "幽靈",
                    ],
                },
            },
            Self::Steel => &TypeData {
                names: NamesData {
                    names: [
                        "はがね",
                        "Steel",
                        "Acier",
                        "Acciaio",
                        "Stahl",
                        "Acero",
                        "강철",
                        "钢",
                        "鋼",
                    ],
                },
            },
            Self::QuestionMark => &TypeData {
                names: NamesData {
                    names: ["？？？", "???", "???", "???", "???", "(?)", "???", "", ""],
                },
            },
            Self::Fire => &TypeData {
                names: NamesData {
                    names: [
                        "ほのお",
                        "Fire",
                        "Feu",
                        "Fuoco",
                        "Feuer",
                        "Fuego",
                        "불꽃",
                        "火",
                        "火",
                    ],
                },
            },
            Self::Water => &TypeData {
                names: NamesData {
                    names: [
                        "みず", "Water", "Eau", "Acqua", "Wasser", "Agua", "물", "水", "水",
                    ],
                },
            },
            Self::Grass => &TypeData {
                names: NamesData {
                    names: [
                        "くさ", "Grass", "Plante", "Erba", "Pflanze", "Planta", "풀", "草", "草",
                    ],
                },
            },
            Self::Electric => &TypeData {
                names: NamesData {
                    names: [
                        "でんき",
                        "Electric",
                        "Électrik",
                        "Elettro",
                        "Elektro",
                        "Eléctrico",
                        "전기",
                        "电",
                        "電",
                    ],
                },
            },
            Self::Psychic => &TypeData {
                names: NamesData {
                    names: [
                        "エスパー",
                        "Psychic",
                        "Psy",
                        "Psico",
                        "Psycho",
                        "Psíquico",
                        "에스퍼",
                        "超能力",
                        "超能力",
                    ],
                },
            },
            Self::Ice => &TypeData {
                names: NamesData {
                    names: [
                        "こおり",
                        "Ice",
                        "Glace",
                        "Ghiaccio",
                        "Eis",
                        "Hielo",
                        "얼음",
                        "冰",
                        "冰",
                    ],
                },
            },
            Self::Dragon => &TypeData {
                names: NamesData {
                    names: [
                        "ドラゴン",
                        "Dragon",
                        "Dragon",
                        "Drago",
                        "Drache",
                        "Dragón",
                        "드래곤",
                        "龙",
                        "龍",
                    ],
                },
            },
            Self::Dark => &TypeData {
                names: NamesData {
                    names: [
                        "あく",
                        "Dark",
                        "Ténèbres",
                        "Buio",
                        "Unlicht",
                        "Siniestro",
                        "악",
                        "恶",
                        "惡",
                    ],
                },
            },
            Self::Fairy => &TypeData {
                names: NamesData {
                    names: [
                        "フェアリー",
                        "Fairy",
                        "Fée",
                        "Folletto",
                        "Fee",
                        "Hada",
                        "페어리",
                        "妖精",
                        "妖精",
                    ],
                },
            },
        }
    }
    pub(crate) fn try_from_gen_1_id(value: u8) -> Result<Self, Error> {
        let value = match value {
            0 => Self::Normal,
            1 => Self::Fighting,
            2 => Self::Flying,
            3 => Self::Poison,
            4 => Self::Ground,
            5 => Self::Rock,
            7 => Self::Bug,
            8 => Self::Ghost,
            20 => Self::Fire,
            21 => Self::Water,
            22 => Self::Grass,
            23 => Self::Electric,
            24 => Self::Psychic,
            25 => Self::Ice,
            26 => Self::Dragon,
            _ => return Err(Error::invalid_argument()),
        };
        Ok(value)
    }
    pub(crate) fn try_into_gen_1_id(&self) -> Result<u8, Error> {
        let value = match self {
            Self::Normal => 0,
            Self::Fighting => 1,
            Self::Flying => 2,
            Self::Poison => 3,
            Self::Ground => 4,
            Self::Rock => 5,
            Self::Bug => 7,
            Self::Ghost => 8,
            Self::Fire => 20,
            Self::Water => 21,
            Self::Grass => 22,
            Self::Electric => 23,
            Self::Psychic => 24,
            Self::Ice => 25,
            Self::Dragon => 26,
            _ => return Err(Error::invalid_argument()),
        };
        Ok(value)
    }
    pub(crate) fn try_from_gen_2_id(value: u8) -> Result<Self, Error> {
        let value = match value {
            0 => Self::Normal,
            1 => Self::Fighting,
            2 => Self::Flying,
            3 => Self::Poison,
            4 => Self::Ground,
            5 => Self::Rock,
            7 => Self::Bug,
            8 => Self::Ghost,
            9 => Self::Steel,
            19 => Self::QuestionMark,
            20 => Self::Fire,
            21 => Self::Water,
            22 => Self::Grass,
            23 => Self::Electric,
            24 => Self::Psychic,
            25 => Self::Ice,
            26 => Self::Dragon,
            27 => Self::Dark,
            _ => return Err(Error::invalid_argument()),
        };
        Ok(value)
    }
    pub(crate) fn try_into_gen_2_id(&self) -> Result<u8, Error> {
        let value = match self {
            Self::Normal => 0,
            Self::Fighting => 1,
            Self::Flying => 2,
            Self::Poison => 3,
            Self::Ground => 4,
            Self::Rock => 5,
            Self::Bug => 7,
            Self::Ghost => 8,
            Self::Steel => 9,
            Self::QuestionMark => 19,
            Self::Fire => 20,
            Self::Water => 21,
            Self::Grass => 22,
            Self::Electric => 23,
            Self::Psychic => 24,
            Self::Ice => 25,
            Self::Dragon => 26,
            Self::Dark => 27,
            _ => return Err(Error::invalid_argument()),
        };
        Ok(value)
    }
}
