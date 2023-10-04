#[derive(Clone, Copy, Debug)]
pub enum NonVolatileStatus {
    Asleep,
    Poisoned,
    BadlyPoisoned,
    Burned,
    Frozen,
    Paralyzed,
}
