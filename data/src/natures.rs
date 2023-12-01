pub use crate::generated::Nature;
use crate::Stat;

impl Nature {
    /// Gets the stat which is increased by 10% for Pokémon with this nature.
    pub fn increased_stat(&self) -> Option<Stat> {
        match self {
            Nature::Hardy | Nature::Docile | Nature::Serious | Nature::Bashful | Nature::Quirky => {
                None
            }
            Nature::Lonely | Nature::Brave | Nature::Adamant | Nature::Naughty => {
                Some(Stat::Attack)
            }
            Nature::Bold | Nature::Relaxed | Nature::Impish | Nature::Lax => Some(Stat::Defense),
            Nature::Timid | Nature::Hasty | Nature::Jolly | Nature::Naive => Some(Stat::Speed),
            Nature::Modest | Nature::Mild | Nature::Quiet | Nature::Rash => {
                Some(Stat::SpecialAttack)
            }
            Nature::Calm | Nature::Gentle | Nature::Sassy | Nature::Careful => {
                Some(Stat::SpecialDefense)
            }
        }
    }

    /// Gets the stat which is decreased by 10% for Pokémon with this nature.
    pub fn decreased_stat(&self) -> Option<Stat> {
        match self {
            Nature::Hardy | Nature::Docile | Nature::Serious | Nature::Bashful | Nature::Quirky => {
                None
            }
            Nature::Bold | Nature::Timid | Nature::Modest | Nature::Calm => Some(Stat::Attack),
            Nature::Lonely | Nature::Hasty | Nature::Mild | Nature::Gentle => Some(Stat::Defense),
            Nature::Brave | Nature::Relaxed | Nature::Quiet | Nature::Sassy => Some(Stat::Speed),
            Nature::Adamant | Nature::Impish | Nature::Jolly | Nature::Careful => {
                Some(Stat::SpecialAttack)
            }
            Nature::Naughty | Nature::Lax | Nature::Naive | Nature::Rash => {
                Some(Stat::SpecialDefense)
            }
        }
    }
}
