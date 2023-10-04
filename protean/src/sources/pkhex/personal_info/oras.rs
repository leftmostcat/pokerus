use super::{PersonalInfo, PersonalInfoRevised};

pub struct PersonalInfoORAS {
    data: Vec<u8>,
}

impl PersonalInfoORAS {
    pub fn _form_count(&self) -> u8 {
        self.data[0x20]
    }

    pub fn _form_stats_index(&self) -> u16 {
        u16::from_le_bytes([self.data[0x1c], self.data[0x1d]])
    }
}

fn type_to_type(type_u8: u8) -> &'static str {
    match type_u8 {
        0 => "Normal",
        1 => "Fighting",
        2 => "Flying",
        3 => "Poison",
        4 => "Ground",
        5 => "Rock",
        6 => "Bug",
        7 => "Ghost",
        8 => "Steel",
        9 => "Fire",
        10 => "Water",
        11 => "Grass",
        12 => "Electric",
        13 => "Psychic",
        14 => "Ice",
        15 => "Dragon",
        16 => "Dark",
        17 => "Fairy",
        value => panic!("unrecognized type value {value}"),
    }
}

impl PersonalInfo for PersonalInfoORAS {
    fn size() -> usize {
        0x50
    }

    fn new(data: &[u8]) -> Self {
        Self {
            data: Vec::from(data),
        }
    }

    fn base_hp(&self) -> u8 {
        self.data[0x00]
    }

    fn base_atk(&self) -> u8 {
        self.data[0x01]
    }

    fn base_def(&self) -> u8 {
        self.data[0x02]
    }

    fn base_spe(&self) -> u8 {
        self.data[0x03]
    }

    fn primary_type(&self) -> &'static str {
        type_to_type(self.data[0x06])
    }

    fn secondary_type(&self) -> &'static str {
        type_to_type(self.data[0x07])
    }

    fn catch_rate(&self) -> u8 {
        todo!()
    }

    fn base_experience_yield(&self) -> u8 {
        todo!()
    }

    fn move_1(&self) -> u8 {
        todo!()
    }

    fn move_2(&self) -> u8 {
        todo!()
    }

    fn move_3(&self) -> u8 {
        todo!()
    }

    fn move_4(&self) -> u8 {
        todo!()
    }

    fn experience_growth_rate(&self) -> &'static str {
        todo!()
    }

    fn ev_yield_hp(&self) -> u8 {
        todo!()
    }

    fn ev_yield_atk(&self) -> u8 {
        todo!()
    }

    fn ev_yield_def(&self) -> u8 {
        todo!()
    }

    fn ev_yield_spe(&self) -> u8 {
        todo!()
    }
}

impl PersonalInfoRevised for PersonalInfoORAS {
    fn base_spa(&self) -> u8 {
        self.data[0x04]
    }

    fn base_spd(&self) -> u8 {
        self.data[0x05]
    }

    fn ev_yield_spa(&self) -> u8 {
        todo!()
    }

    fn ev_yield_spd(&self) -> u8 {
        todo!()
    }

    fn gender_ratio(&self) -> &'static str {
        todo!()
    }

    fn hatch_cycles(&self) -> u8 {
        todo!()
    }
}
