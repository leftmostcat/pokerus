use core::time::Duration;

use alloc::vec::Vec;
use common::Error;

use crate::Pokemon;

pub trait Save<'a, P: Pokemon + 'a> {
    type PokemonBox;

    fn trainer_id(&self) -> u32;
    fn trainer_name(&self) -> Result<&str, Error>;
    fn money(&self) -> u32;

    fn play_time(&self) -> Duration;

    fn party(&self) -> &[P];

    fn box_count(&self) -> usize;
    fn current_box_idx(&self) -> usize;

    fn boxes(&self) -> &[Self::PokemonBox];
}

pub struct ListBox<P: Pokemon> {
    pokemon: Vec<P>,
}

impl<P: Pokemon> AsRef<[P]> for ListBox<P> {
    fn as_ref(&self) -> &[P] {
        &self.pokemon
    }
}

impl<P: Pokemon> AsMut<[P]> for ListBox<P> {
    fn as_mut(&mut self) -> &mut [P] {
        &mut self.pokemon
    }
}

impl<P: Pokemon> FromIterator<P> for ListBox<P> {
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Self {
            pokemon: iter.into_iter().collect(),
        }
    }
}

pub struct StandardBox<P: Pokemon> {
    pokemon: [Option<P>; 30],
}

impl<P: Pokemon> AsRef<[Option<P>]> for StandardBox<P> {
    fn as_ref(&self) -> &[Option<P>] {
        &self.pokemon
    }
}
