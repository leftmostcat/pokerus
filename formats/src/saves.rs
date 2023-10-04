use core::{slice::Iter, time::Duration};

use common::Error;

use crate::Pokemon;

pub trait Save<'a, P: Pokemon + 'a> {
    fn trainer_id(&self) -> u32;
    fn trainer_name(&self) -> Result<&str, Error>;
    fn money(&self) -> u32;

    fn play_time(&self) -> Duration;

    fn party(&self) -> &[P];

    fn box_count(&self) -> usize;
    fn current_box_idx(&self) -> usize;
}

pub trait PokemonBox<'a, P: Pokemon + 'a> {
    type Iter: Iterator<Item = &'a Option<P>>;

    fn iter(&'a self) -> Self::Iter;
}

pub struct StandardBox<P: Pokemon> {
    pokemon: [Option<P>; 30],
}

impl<'a, P: Pokemon + 'a> PokemonBox<'a, P> for StandardBox<P> {
    type Iter = Iter<'a, Option<P>>;

    fn iter(&'a self) -> Self::Iter {
        self.pokemon.iter()
    }
}
