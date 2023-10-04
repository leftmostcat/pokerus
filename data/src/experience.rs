use common::Error;

use crate::generated::experience::*;

#[derive(Clone, Copy, Debug)]
pub enum ExperienceGrowthRate {
    Erratic,
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
    Fluctuating,
}

pub fn calculate_level(experience: u32, growth_rate: ExperienceGrowthRate) -> Result<u8, Error> {
    match growth_rate {
        ExperienceGrowthRate::Erratic => calculate_level_erratic(experience),
        ExperienceGrowthRate::Fast => calculate_level_fast(experience),
        ExperienceGrowthRate::MediumFast => calculate_level_medium_fast(experience),
        ExperienceGrowthRate::MediumSlow => calculate_level_medium_slow(experience),
        ExperienceGrowthRate::Slow => calculate_level_slow(experience),
        ExperienceGrowthRate::Fluctuating => calculate_level_fluctuating(experience),
    }
}

pub fn min_experience_for_level(
    level: u8,
    growth_rate: ExperienceGrowthRate,
) -> Result<u32, Error> {
    match growth_rate {
        ExperienceGrowthRate::Erratic => min_experience_for_level_erratic(level),
        ExperienceGrowthRate::Fast => min_experience_for_level_fast(level),
        ExperienceGrowthRate::MediumFast => min_experience_for_level_medium_fast(level),
        ExperienceGrowthRate::MediumSlow => min_experience_for_level_medium_slow(level),
        ExperienceGrowthRate::Slow => min_experience_for_level_slow(level),
        ExperienceGrowthRate::Fluctuating => min_experience_for_level_fluctuating(level),
    }
}
