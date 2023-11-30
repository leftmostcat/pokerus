use crate::{
    conversions::{get_gender_ratio_variant, get_growth_rate_variant, get_type_variant},
    rse_frlg::gen_3_type_id_to_standard,
};

use super::{PersonalInfo, PersonalInfoRevised};

pub struct PersonalInfoRSEFRLG {
    data: Vec<u8>,
}

impl PersonalInfo for PersonalInfoRSEFRLG {
    fn size() -> usize {
        0x1c
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
        get_type_variant(gen_3_type_id_to_standard(self.data[0x06]))
    }

    fn secondary_type(&self) -> &'static str {
        get_type_variant(gen_3_type_id_to_standard(self.data[0x07]))
    }

    fn catch_rate(&self) -> u8 {
        self.data[0x08]
    }

    fn base_experience_yield(&self) -> u8 {
        self.data[0x09]
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
        get_growth_rate_variant(self.data[0x13])
    }

    fn ev_yield_hp(&self) -> u8 {
        self.data[0x0a] & 0x03
    }

    fn ev_yield_atk(&self) -> u8 {
        (self.data[0x0a] >> 2) & 0x03
    }

    fn ev_yield_def(&self) -> u8 {
        (self.data[0x0a] >> 4) & 0x03
    }

    fn ev_yield_spe(&self) -> u8 {
        (self.data[0x0a] >> 6) & 0x03
    }
}

impl PersonalInfoRevised for PersonalInfoRSEFRLG {
    fn base_spa(&self) -> u8 {
        self.data[0x04]
    }

    fn base_spd(&self) -> u8 {
        self.data[0x05]
    }

    fn ev_yield_spa(&self) -> u8 {
        self.data[0x0b] & 0x03
    }

    fn ev_yield_spd(&self) -> u8 {
        (self.data[0x0b] >> 2) & 0x03
    }

    fn gender_ratio(&self) -> &'static str {
        get_gender_ratio_variant(self.data[0x10])
    }

    fn hatch_cycles(&self) -> u8 {
        self.data[0x11]
    }
}
