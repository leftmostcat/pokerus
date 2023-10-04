use std::{fs, path::Path};

use crate::{conversions::get_type_variant, rgby::gen_1_type_id_to_standard};

pub struct MoveDataRGBY {
    pub idx: u8,
    _effect: u8,
    pub power: u8,
    move_type: u8,
    accuracy: u8,
    pub base_pp: u8,
}

impl MoveDataRGBY {
    pub fn accuracy(&self) -> u8 {
        (self.accuracy as f32 * 100.0 / 255.0).ceil() as u8
    }

    pub fn move_type(&self) -> &'static str {
        get_type_variant(gen_1_type_id_to_standard(self.move_type))
    }
}

pub fn read_moves(source_dir: &Path) -> Vec<MoveDataRGBY> {
    const MOVE_DATA_SIZE: usize = 0x06;

    let path = source_dir.join("rgby/moves.dat");
    let data = fs::read(path).unwrap();
    assert_eq!(data.len() % MOVE_DATA_SIZE, 0);

    data.chunks_exact(MOVE_DATA_SIZE)
        .map(|data| MoveDataRGBY {
            idx: data[0],
            _effect: data[1],
            power: data[2],
            move_type: data[3],
            accuracy: data[4],
            base_pp: data[5],
        })
        .collect()
}
