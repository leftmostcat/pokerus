pub(crate) trait DataReader {
    /// Reads an unsigned 16-bit int from big endian data.
    fn read_u16_be(&self, offset: usize) -> u16;

    /// Reads an unsigned 24-bit int from big endian data.
    fn read_u24_be(&self, offset: usize) -> u32;
}

impl DataReader for &[u8] {
    fn read_u16_be(&self, offset: usize) -> u16 {
        u16::from_be_bytes([self[offset], self[offset + 1]])
    }

    fn read_u24_be(&self, offset: usize) -> u32 {
        u32::from_be_bytes([0, self[offset], self[offset + 1], self[offset + 2]])
    }
}
