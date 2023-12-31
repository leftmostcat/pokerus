use common::Error;

pub use crate::generated::Species;
use crate::{ExperienceGrowthRate, GameSet, GenderRatio, Language, NamesData, Type};

impl Species {
    pub fn try_from_species_and_form_id(
        species_id: u16,
        form_id: u8,
        game_set: GameSet,
    ) -> Result<Self, Error> {
        if species_id == 0 {
            return Err(Error::invalid_argument());
        }

        match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => {
                if species_id > 0xbe || form_id != 0 {
                    return Err(Error::invalid_argument());
                }

                Self::try_from_gen_1_species_id(species_id as u8)
            }
            GameSet::GoldSilver | GameSet::Crystal => {
                Self::try_from_gen_2_species_and_form_id(species_id as u8, form_id)
            }
            GameSet::RubySapphire => todo!(),
            GameSet::Emerald => todo!(),
            GameSet::FireRed => todo!(),
            GameSet::LeafGreen => todo!(),
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        }
    }

    pub fn national_dex_id(&self) -> u16 {
        self.species_data().national_dex_id
    }

    pub fn name(&self, language: Language) -> &'static str {
        self.species_data().names.get(language)
    }

    pub fn form_name(&self, language: Language) -> &'static str {
        self.form_data().names.get(language)
    }

    pub fn primary_type(&self, game_set: GameSet) -> Result<Type, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue => self.species_data_rb()?.primary_type,
            GameSet::Yellow => self.species_data_y()?.primary_type,
            GameSet::GoldSilver => self.species_data_gs()?.primary_type,
            GameSet::Crystal => self.species_data_c()?.primary_type,
            GameSet::RubySapphire => self.species_data_rs()?.primary_type,
            GameSet::Emerald => self.species_data_e()?.primary_type,
            GameSet::FireRed => self.species_data_fr()?.primary_type,
            GameSet::LeafGreen => self.species_data_lg()?.primary_type,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn secondary_type(&self, game_set: GameSet) -> Result<Type, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue => self.species_data_rb()?.secondary_type,
            GameSet::Yellow => self.species_data_y()?.secondary_type,
            GameSet::GoldSilver => self.species_data_gs()?.secondary_type,
            GameSet::Crystal => self.species_data_c()?.secondary_type,
            GameSet::RubySapphire => self.species_data_rs()?.secondary_type,
            GameSet::Emerald => self.species_data_e()?.secondary_type,
            GameSet::FireRed => self.species_data_fr()?.secondary_type,
            GameSet::LeafGreen => self.species_data_lg()?.secondary_type,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn experience_growth_rate(&self, game_set: GameSet) -> Result<ExperienceGrowthRate, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue => self.species_data_rb()?.experience_growth_rate,
            GameSet::Yellow => self.species_data_y()?.experience_growth_rate,
            GameSet::GoldSilver => self.species_data_gs()?.experience_growth_rate,
            GameSet::Crystal => self.species_data_c()?.experience_growth_rate,
            GameSet::RubySapphire => self.species_data_rs()?.experience_growth_rate,
            GameSet::Emerald => self.species_data_e()?.experience_growth_rate,
            GameSet::FireRed => self.species_data_fr()?.experience_growth_rate,
            GameSet::LeafGreen => self.species_data_lg()?.experience_growth_rate,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn base_hit_points(&self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue => self.species_data_rb()?.base_hp,
            GameSet::Yellow => self.species_data_y()?.base_hp,
            GameSet::GoldSilver => self.species_data_gs()?.base_hp,
            GameSet::Crystal => self.species_data_c()?.base_hp,
            GameSet::RubySapphire => self.species_data_rs()?.base_hp,
            GameSet::Emerald => self.species_data_e()?.base_hp,
            GameSet::FireRed => self.species_data_fr()?.base_hp,
            GameSet::LeafGreen => self.species_data_lg()?.base_hp,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn base_attack(&self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue => self.species_data_rb()?.base_atk,
            GameSet::Yellow => self.species_data_y()?.base_atk,
            GameSet::GoldSilver => self.species_data_gs()?.base_atk,
            GameSet::Crystal => self.species_data_c()?.base_atk,
            GameSet::RubySapphire => self.species_data_rs()?.base_atk,
            GameSet::Emerald => self.species_data_e()?.base_atk,
            GameSet::FireRed => self.species_data_fr()?.base_atk,
            GameSet::LeafGreen => self.species_data_lg()?.base_atk,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn base_defense(&self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue => self.species_data_rb()?.base_def,
            GameSet::Yellow => self.species_data_y()?.base_def,
            GameSet::GoldSilver => self.species_data_gs()?.base_def,
            GameSet::Crystal => self.species_data_c()?.base_def,
            GameSet::RubySapphire => self.species_data_rs()?.base_def,
            GameSet::Emerald => self.species_data_e()?.base_def,
            GameSet::FireRed => self.species_data_fr()?.base_def,
            GameSet::LeafGreen => self.species_data_lg()?.base_def,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn base_special(&self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue => self.species_data_rb()?.base_spc,
            GameSet::Yellow => self.species_data_y()?.base_spc,
            _ => return Err(Error::invalid_argument()),
        };

        Ok(value)
    }

    pub fn base_special_attack(&self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => return Err(Error::invalid_argument()),
            GameSet::GoldSilver => self.species_data_gs()?.base_spa,
            GameSet::Crystal => self.species_data_c()?.base_spa,
            GameSet::RubySapphire => self.species_data_rs()?.base_spa,
            GameSet::Emerald => self.species_data_e()?.base_spa,
            GameSet::FireRed => self.species_data_fr()?.base_spa,
            GameSet::LeafGreen => self.species_data_lg()?.base_spa,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn base_special_defense(&self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => return Err(Error::invalid_argument()),
            GameSet::GoldSilver => self.species_data_gs()?.base_spd,
            GameSet::Crystal => self.species_data_c()?.base_spd,
            GameSet::RubySapphire => self.species_data_rs()?.base_spd,
            GameSet::Emerald => self.species_data_e()?.base_spd,
            GameSet::FireRed => self.species_data_fr()?.base_spd,
            GameSet::LeafGreen => self.species_data_lg()?.base_spd,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn base_speed(&self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue => self.species_data_rb()?.base_spe,
            GameSet::Yellow => self.species_data_y()?.base_spe,
            GameSet::GoldSilver => self.species_data_gs()?.base_spe,
            GameSet::Crystal => self.species_data_c()?.base_spe,
            GameSet::RubySapphire => self.species_data_rs()?.base_spe,
            GameSet::Emerald => self.species_data_e()?.base_spe,
            GameSet::FireRed => self.species_data_fr()?.base_spe,
            GameSet::LeafGreen => self.species_data_lg()?.base_spe,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn catch_rate(&self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue => self.species_data_rb()?.catch_rate,
            GameSet::Yellow => self.species_data_y()?.catch_rate,
            GameSet::GoldSilver => self.species_data_gs()?.catch_rate,
            GameSet::Crystal => self.species_data_c()?.catch_rate,
            GameSet::RubySapphire => self.species_data_rs()?.catch_rate,
            GameSet::Emerald => self.species_data_e()?.catch_rate,
            GameSet::FireRed => self.species_data_fr()?.catch_rate,
            GameSet::LeafGreen => self.species_data_lg()?.catch_rate,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }

    pub fn gender_ratio(&self, game_set: GameSet) -> Result<GenderRatio, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => return Err(Error::invalid_argument()),
            GameSet::GoldSilver => self.species_data_gs()?.gender_ratio,
            GameSet::Crystal => self.species_data_c()?.gender_ratio,
            GameSet::RubySapphire => self.species_data_rs()?.gender_ratio,
            GameSet::Emerald => self.species_data_e()?.gender_ratio,
            GameSet::FireRed => self.species_data_fr()?.gender_ratio,
            GameSet::LeafGreen => self.species_data_lg()?.gender_ratio,
            GameSet::DiamondPearl => todo!(),
            GameSet::Platinum => todo!(),
            GameSet::HeartGoldSoulSilver => todo!(),
            GameSet::BlackWhite => todo!(),
            GameSet::Black2White2 => todo!(),
            GameSet::XY => todo!(),
            GameSet::OmegaRubyAlphaSapphire => todo!(),
            GameSet::SunMoon => todo!(),
            GameSet::UltraSunUltraMoon => todo!(),
            GameSet::LetsGoPikachuEevee => todo!(),
            GameSet::SwordShield => todo!(),
            GameSet::BrilliantDiamondShiningPearl => todo!(),
            GameSet::LegendsArceus => todo!(),
            GameSet::ScarletViolet => todo!(),
        };

        Ok(value)
    }
}

pub(crate) struct SpeciesData {
    pub(crate) national_dex_id: u16,
    pub(crate) names: NamesData,
}

pub(crate) struct FormData {
    pub(crate) names: NamesData,
}

pub(crate) struct SpeciesDataRGBY {
    pub base_hp: u8,
    pub base_atk: u8,
    pub base_def: u8,
    pub base_spc: u8,
    pub base_spe: u8,
    pub primary_type: Type,
    pub secondary_type: Type,
    pub catch_rate: u8,
    pub experience_growth_rate: ExperienceGrowthRate,
}

pub(crate) struct SpeciesDataGSC {
    pub base_hp: u8,
    pub base_atk: u8,
    pub base_def: u8,
    pub base_spa: u8,
    pub base_spd: u8,
    pub base_spe: u8,
    pub primary_type: Type,
    pub secondary_type: Type,
    pub catch_rate: u8,
    pub experience_growth_rate: ExperienceGrowthRate,
    pub gender_ratio: GenderRatio,
}

pub(crate) struct SpeciesDataRSEFRLG {
    pub base_hp: u8,
    pub base_atk: u8,
    pub base_def: u8,
    pub base_spa: u8,
    pub base_spd: u8,
    pub base_spe: u8,
    pub primary_type: Type,
    pub secondary_type: Type,
    pub catch_rate: u8,
    pub experience_growth_rate: ExperienceGrowthRate,
    pub gender_ratio: GenderRatio,
}
