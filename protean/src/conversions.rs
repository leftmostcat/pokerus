pub(super) fn get_gender_ratio_variant(gender_ratio: u8) -> &'static str {
    match gender_ratio {
        0 => "MaleOnly",
        31 => "SevenMaleToOneFemale",
        63 => "ThreeMaleToOneFemale",
        127 => "OneToOne",
        191 => "ThreeFemaleToOneMale",
        225 => "SevenFemaleToOneMale",
        254 => "FemaleOnly",
        255 => "GenderUnknown",

        _ => panic!("Invalid gender ratio value {gender_ratio}"),
    }
}

pub(super) fn get_growth_rate_variant(growth_rate: u8) -> &'static str {
    match growth_rate {
        0 => "MediumFast",
        1 => "Erratic",
        2 => "Fluctuating",
        3 => "MediumSlow",
        4 => "Fast",
        5 => "Slow",

        _ => panic!("Invalid growth rate value {growth_rate}"),
    }
}

pub(crate) fn get_type_variant(type_index: u8) -> &'static str {
    match type_index {
        0 => "Normal",
        1 => "Fighting",
        2 => "Flying",
        3 => "Poison",
        4 => "Ground",
        5 => "Rock",
        6 => "Bug",
        7 => "Ghost",
        8 => "Steel",

        10 => "Fire",
        11 => "Water",
        12 => "Grass",
        13 => "Electric",
        14 => "Psychic",
        15 => "Ice",
        16 => "Dragon",
        17 => "Dark",
        18 => "Fairy",

        _ => panic!("Invalid type value {type_index}"),
    }
}
