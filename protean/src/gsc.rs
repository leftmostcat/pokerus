pub(crate) fn gen_2_type_id_to_standard(id: u8) -> u8 {
    match id {
        0x00..=0x05 => id,
        0x07..=0x10 => id - 1,
        0x14..=0x1b => id - 10,

        _ => panic!("Invalid gen 1 type ID {id}"),
    }
}

pub(crate) fn type_id_to_gen_2_idx(idx: u8) -> Option<u8> {
    let id = match idx {
        0..=5 => idx,
        6..=8 => idx + 1,
        9..=17 => idx + 10,

        _ => return None,
    };

    Some(id)
}
