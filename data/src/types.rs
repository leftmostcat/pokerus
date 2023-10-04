use common::Error;

pub use crate::generated::Type;
use crate::{GameSet, Language, NamesData};

impl Type {
    pub fn try_from_type_id(id: u8, game_set: GameSet) -> Result<Self, Error> {
        match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => Self::try_from_gen_1_id(id),
            GameSet::GoldSilver | GameSet::Crystal => Self::try_from_gen_2_id(id),
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

    pub fn try_into_type_id(&self, game_set: GameSet) -> Result<u8, Error> {
        match game_set {
            GameSet::RedGreenBlue | GameSet::Yellow => self.try_into_gen_1_id(),
            GameSet::GoldSilver | GameSet::Crystal => self.try_into_gen_2_id(),
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

    pub fn name(self, language: Language) -> &'static str {
        self.data().names.get(language)
    }
}

pub(crate) struct TypeData {
    pub(crate) names: NamesData,
}
