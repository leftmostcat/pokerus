use crate::{
    conversions::{get_growth_rate_variant, get_type_variant},
    rgby::gen_1_type_id_to_standard,
};

use super::PersonalInfo;

pub trait PersonalInfoGen1: PersonalInfo {
    fn base_spc(&self) -> u8;
    fn ev_yield_spc(&self) -> u8;
}

pub struct PersonalInfoRGBY {
    data: Vec<u8>,
}

impl PersonalInfo for PersonalInfoRGBY {
    fn size() -> usize {
        0x1c
    }

    fn new(data: &[u8]) -> Self {
        Self {
            data: Vec::from(data),
        }
    }

    fn base_hp(&self) -> u8 {
        self.data[0x01]
    }

    fn base_atk(&self) -> u8 {
        self.data[0x02]
    }

    fn base_def(&self) -> u8 {
        self.data[0x03]
    }

    fn base_spe(&self) -> u8 {
        self.data[0x04]
    }

    fn primary_type(&self) -> &'static str {
        get_type_variant(gen_1_type_id_to_standard(self.data[0x06]))
    }

    fn secondary_type(&self) -> &'static str {
        get_type_variant(gen_1_type_id_to_standard(self.data[0x07]))
    }

    fn catch_rate(&self) -> u8 {
        self.data[0x08]
    }

    fn base_experience_yield(&self) -> u8 {
        self.data[0x09]
    }

    fn move_1(&self) -> u8 {
        self.data[0x0f]
    }

    fn move_2(&self) -> u8 {
        self.data[0x10]
    }

    fn move_3(&self) -> u8 {
        self.data[0x11]
    }

    fn move_4(&self) -> u8 {
        self.data[0x12]
    }

    fn experience_growth_rate(&self) -> &'static str {
        get_growth_rate_variant(self.data[0x13])
    }

    fn ev_yield_hp(&self) -> u8 {
        self.base_hp()
    }

    fn ev_yield_atk(&self) -> u8 {
        self.base_atk()
    }

    fn ev_yield_def(&self) -> u8 {
        self.base_def()
    }

    fn ev_yield_spe(&self) -> u8 {
        self.base_spe()
    }
}

impl PersonalInfoGen1 for PersonalInfoRGBY {
    fn base_spc(&self) -> u8 {
        self.data[0x05]
    }

    fn ev_yield_spc(&self) -> u8 {
        self.base_spc()
    }
}
