pub mod gsc;
pub mod oras;
pub mod rgby;

pub struct PersonalTable<I: PersonalInfo> {
    pub info_blocks: Vec<I>,
}

impl<I: PersonalInfo> PersonalTable<I> {
    pub fn new(data: Vec<u8>) -> Self {
        let info_blocks = data.chunks_exact(I::size());

        assert_eq!(info_blocks.remainder().len(), 0);

        let info_blocks = info_blocks.map(|data| I::new(data)).collect();

        Self { info_blocks }
    }
}

pub trait PersonalInfo {
    fn size() -> usize;

    fn new(data: &[u8]) -> Self;

    fn base_hp(&self) -> u8;
    fn base_atk(&self) -> u8;
    fn base_def(&self) -> u8;
    fn base_spe(&self) -> u8;

    fn primary_type(&self) -> &'static str;
    fn secondary_type(&self) -> &'static str;

    fn catch_rate(&self) -> u8;
    fn base_experience_yield(&self) -> u8;

    fn move_1(&self) -> u8;
    fn move_2(&self) -> u8;
    fn move_3(&self) -> u8;
    fn move_4(&self) -> u8;

    fn experience_growth_rate(&self) -> &'static str;

    fn ev_yield_hp(&self) -> u8;
    fn ev_yield_atk(&self) -> u8;
    fn ev_yield_def(&self) -> u8;
    fn ev_yield_spe(&self) -> u8;
}

pub trait PersonalInfoRevised: PersonalInfo {
    fn base_spa(&self) -> u8;
    fn base_spd(&self) -> u8;

    fn ev_yield_spa(&self) -> u8;
    fn ev_yield_spd(&self) -> u8;

    fn gender_ratio(&self) -> &'static str;

    fn hatch_cycles(&self) -> u8;
}
