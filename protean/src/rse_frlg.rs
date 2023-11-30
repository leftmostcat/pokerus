pub(crate) fn gen_3_type_id_to_standard(id: u8) -> u8 {
    match id {
        0x00..=0x08 => id,
        0x09..=0x10 => id + 1,

        _ => panic!("Invalid gen 3 type ID {id}"),
    }
}
