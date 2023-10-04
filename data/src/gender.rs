#[derive(Clone, Copy, Debug)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

#[derive(Clone, Copy, Debug)]
pub enum GenderRatio {
    MaleOnly,
    SevenMaleToOneFemale,
    ThreeMaleToOneFemale,

    OneToOne,

    ThreeFemaleToOneMale,
    SevenFemaleToOneMale,
    FemaleOnly,

    GenderUnknown,
}
