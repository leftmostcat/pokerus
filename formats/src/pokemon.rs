use common::Error;
use pokerus_data::{Gender, Item, Move, Nature, NonVolatileStatus, PokerusStrain, Species};

pub trait Pokemon {
    fn species(&self) -> Result<Species, Error>;

    fn nickname(&self) -> Result<&str, Error>;
    fn is_nicknamed(&self) -> bool;

    fn original_trainer_id(&self) -> u16;
    fn original_trainer(&self) -> Result<&str, Error>;

    fn experience(&self) -> u32;
    fn level(&self) -> Result<u8, Error>;

    fn move_values(&self, idx: usize) -> Result<Option<PokemonMove>, Error>;

    fn status_condition(&self) -> Result<Option<NonVolatileStatus>, Error>;

    fn iv_hp(&self) -> u8;
    fn iv_atk(&self) -> u8;
    fn iv_def(&self) -> u8;
    fn iv_spe(&self) -> u8;

    fn ev_hp(&self) -> u16;
    fn ev_atk(&self) -> u16;
    fn ev_def(&self) -> u16;
    fn ev_spe(&self) -> u16;

    fn current_hp(&self) -> u16;

    fn max_hp(&self) -> Result<u16, Error>;
    fn atk(&self) -> Result<u16, Error>;
    fn def(&self) -> Result<u16, Error>;
    fn spe(&self) -> Result<u16, Error>;
}

pub trait HasFriendship: Pokemon {
    fn friendship(&self) -> u8;
}

pub trait HasInternalSpc: Pokemon {
    fn iv_spc(&self) -> u8;
    fn ev_spc(&self) -> u16;
}

pub trait HasSpc: Pokemon {
    fn spc(&self) -> Result<u16, Error>;
}

pub trait HasGender: Pokemon {
    fn gender(&self) -> Result<Gender, Error>;
}

pub trait HasSpaSpd: Pokemon {
    fn spa(&self) -> Result<u16, Error>;
    fn spd(&self) -> Result<u16, Error>;
}

pub trait HasInternalSpaSpd: Pokemon {
    fn iv_spa(&self) -> u8;
    fn iv_spd(&self) -> u8;

    fn ev_spa(&self) -> u16;
    fn ev_spd(&self) -> u16;
}

pub trait HasNature: Pokemon {
    fn nature(&self) -> Result<Nature, Error>;
}

pub trait HasShininess: Pokemon {
    fn is_shiny(&self) -> bool;
}

pub trait CanHavePokerus: Pokemon {
    fn pokerus_days_left(&self) -> u8;
    fn pokerus_strain(&self) -> Option<PokerusStrain>;

    fn has_or_had_pokerus(&self) -> bool {
        self.pokerus_strain().is_some()
    }

    fn is_cured_of_pokerus(&self) -> bool {
        self.pokerus_days_left() == 0
    }
}

pub struct PokemonMove {
    pub value: Move,
    pub current_pp: u8,
    pub pp_ups: u8,
}

pub struct InventoryItem {
    pub value: Item,
    pub count: u16,
}
