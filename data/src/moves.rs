use common::Error;

pub use crate::generated::Move;
use crate::{GameSet, Language, NamesData, Type};

impl Move {
    pub fn name(self, language: Language) -> &'static str {
        self.move_data().names.get(language)
    }

    pub fn base_pp(self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => self.move_data_for_rgby()?.max_pp,
            GameSet::GoldSilver => todo!(),
            GameSet::Crystal => todo!(),
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
        };

        Ok(value)
    }

    pub fn move_type(self, game_set: GameSet) -> Result<Type, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => self.move_data_for_rgby()?.move_type,
            GameSet::GoldSilver => todo!(),
            GameSet::Crystal => todo!(),
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
        };

        Ok(value)
    }

    pub fn power(self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => self.move_data_for_rgby()?.power,
            GameSet::GoldSilver => todo!(),
            GameSet::Crystal => todo!(),
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
        };

        Ok(value)
    }

    pub fn accuracy(self, game_set: GameSet) -> Result<u8, Error> {
        let value = match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => self.move_data_for_rgby()?.accuracy,
            GameSet::GoldSilver => todo!(),
            GameSet::Crystal => todo!(),
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
        };

        Ok(value)
    }
}

pub(crate) struct MoveData {
    pub(crate) names: NamesData,
}

pub struct MoveDataRGBY {
    pub max_pp: u8,
    pub move_type: Type,
    pub power: u8,
    pub accuracy: u8,
}
