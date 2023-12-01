/// A `Stat` is one of the values unique to each Pokémon used in battle
/// calculations.
///
/// This version represents the set of stats used in Generations III and later
/// for all values and in Generation II for species base values and user-facing
/// stat values.
///
/// For the set of stats used for all values in Generation I and for effort
/// values and individual/determinant values in Generation II, see
/// [`StatWithSpecial`].
pub enum Stat {
    HitPoints,
    Attack,
    Defense,
    Speed,
    SpecialAttack,
    SpecialDefense,
}

/// A `StatWithSpecial` is one of the values unique to each Pokémon used in
/// battle calculations.
///
/// This version represents the set of stats used in Generation I for all values
/// and in Generation II for effort values and individual/determinant values.
///
/// For the set of stats used for all values in Generations III and later and
/// for species base values and user-facing stat values in Generation II, see
/// [`Stat`].
pub enum StatWithSpecial {
    HitPoints,
    Attack,
    Defense,
    Speed,
    Special,
}
