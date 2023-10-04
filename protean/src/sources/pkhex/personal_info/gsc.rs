use crate::{
    conversions::{get_gender_ratio_variant, get_growth_rate_variant, get_type_variant},
    gsc::gen_2_type_id_to_standard,
};

use super::{PersonalInfo, PersonalInfoRevised};

pub struct PersonalInfoGSC {
    data: Vec<u8>,
}

impl PersonalInfo for PersonalInfoGSC {
    fn size() -> usize {
        0x20
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
        get_type_variant(gen_2_type_id_to_standard(self.data[0x07]))
    }

    fn secondary_type(&self) -> &'static str {
        get_type_variant(gen_2_type_id_to_standard(self.data[0x08]))
    }

    fn catch_rate(&self) -> u8 {
        self.data[0x09]
    }

    fn base_experience_yield(&self) -> u8 {
        self.data[0x0a]
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
        get_growth_rate_variant(self.data[0x16])
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

impl PersonalInfoRevised for PersonalInfoGSC {
    fn base_spa(&self) -> u8 {
        self.data[0x05]
    }

    fn base_spd(&self) -> u8 {
        self.data[0x06]
    }

    fn ev_yield_spa(&self) -> u8 {
        self.base_spa()
    }

    fn ev_yield_spd(&self) -> u8 {
        self.base_spd()
    }

    fn gender_ratio(&self) -> &'static str {
        get_gender_ratio_variant(self.data[0x0d])
    }

    fn hatch_cycles(&self) -> u8 {
        todo!()
    }
}
