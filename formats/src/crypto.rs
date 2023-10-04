use alloc::vec::Vec;

const GEN_6_BLOCK_SIZE: usize = 56;

/// Performs in-place decryption of a generation 6 Pokémon.
///
/// Callers are responsible for ensuring the input data is correctly sized.
pub fn decrypt_gen_6(data: &mut [u8]) {
    const ENCRYPTED_START: usize = 8;
    const ENCRYPTED_END: usize = ENCRYPTED_START + GEN_6_BLOCK_SIZE * 4;
    assert!(data.len() >= ENCRYPTED_END);

    // The Pokémon's personality value, the first four bytes of its data, is
    // used as the initial seed for the XOR cipher's key generator.
    let personality_value = [data[0], data[1], data[2], data[3]];
    let personality_value = u32::from_le_bytes(personality_value);

    let encrypted = &mut data[ENCRYPTED_START..ENCRYPTED_END];
    xor_mask(encrypted, personality_value);

    let ordering = (personality_value >> 13) & 31;
    transpose_blocks(encrypted, TransposeOrdering::inverse(ordering.into()));

    if data.len() > ENCRYPTED_END {
        // Pokémon in the player's party have an additional block of
        // pre-computed stats after the main data. This block is XOR ciphered,
        // but does not undergo block transposition.
        xor_mask(&mut data[ENCRYPTED_END..], personality_value);
    }
}

/// Reorders the input data as a permutation of four blocks, with the exact
/// permutation specified by the provided ordering.
///
/// Callers are responsible for verifying that the input data has a length in
/// bytes which is divisible by 4.
///
/// # Performance
/// Note that this function may allocate up to the length of the input data as
/// a buffer. It may also perform two copies of the length of the input data.
fn transpose_blocks(data: &mut [u8], ordering: TransposeOrdering) {
    assert_eq!(data.len() % 4, 0);
    let block_size = data.len() / 4;

    if matches!(ordering, TransposeOrdering::ABCD) {
        // This is the identity permutation, so no work is needed.
        return;
    }

    // Arrange the blocks according to the specified permutation.
    let (a, b, c, d) = (
        &data[0..block_size],
        &data[block_size..block_size * 2],
        &data[block_size * 2..block_size * 3],
        &data[block_size * 3..],
    );

    // We allocate here in order to prevent overwriting values before reading
    // them.
    //
    // This could likely be made significantly more memory-efficient, but the
    // performance tradeoffs haven't been explored.
    let buffer: Vec<_> = match ordering {
        TransposeOrdering::ABDC => [a, b, d, c],
        TransposeOrdering::ACBD => [a, c, b, d],
        TransposeOrdering::ACDB => [a, c, d, b],
        TransposeOrdering::ADBC => [a, d, b, c],
        TransposeOrdering::ADCB => [a, d, c, b],
        TransposeOrdering::BACD => [b, a, c, d],
        TransposeOrdering::BADC => [b, a, d, c],
        TransposeOrdering::BCAD => [b, c, a, d],
        TransposeOrdering::BCDA => [b, c, d, a],
        TransposeOrdering::BDAC => [b, d, a, c],
        TransposeOrdering::BDCA => [b, d, c, a],
        TransposeOrdering::CABD => [c, a, b, d],
        TransposeOrdering::CADB => [c, a, d, b],
        TransposeOrdering::CBAD => [c, b, a, d],
        TransposeOrdering::CBDA => [c, b, d, a],
        TransposeOrdering::CDAB => [c, d, a, b],
        TransposeOrdering::CDBA => [c, d, b, a],
        TransposeOrdering::DABC => [d, a, b, c],
        TransposeOrdering::DACB => [d, a, c, b],
        TransposeOrdering::DBAC => [d, b, a, c],
        TransposeOrdering::DBCA => [d, b, c, a],
        TransposeOrdering::DCAB => [d, c, a, b],
        TransposeOrdering::DCBA => [d, c, b, a],

        TransposeOrdering::ABCD => {
            panic!("Expected transposition ordering to be handled already. This is a library bug.")
        }
    }
    .into_iter()
    .flatten()
    .copied()
    .collect();

    data.clone_from_slice(&buffer);
}

/// Represents the possible permutations over four elements and the
/// corresponding values which produce these permutations as used in the block
/// transposition algorithm.
#[allow(clippy::upper_case_acronyms)]
enum TransposeOrdering {
    ABCD = 0,
    ABDC = 1,
    ACBD = 2,
    ACDB = 3,
    ADBC = 4,
    ADCB = 5,

    BACD = 6,
    BADC = 7,
    BCAD = 8,
    BCDA = 9,
    BDAC = 10,
    BDCA = 11,

    CABD = 12,
    CADB = 13,
    CBAD = 14,
    CBDA = 15,
    CDAB = 16,
    CDBA = 17,

    DABC = 18,
    DACB = 19,
    DBAC = 20,
    DBCA = 21,
    DCAB = 22,
    DCBA = 23,
}

impl TransposeOrdering {
    /// Gets the ordering for which applying [`transpose_blocks()`] will invert
    /// application with the ordering on which this is called.
    #[inline(always)]
    fn inverse(self) -> Self {
        // The match arms are by nature symmetrical, since
        // `inverse(inverse(A)) == A`, but I can't think of a better way to do
        // this.
        match self {
            TransposeOrdering::ACDB => Self::ADBC,
            TransposeOrdering::ADBC => Self::ACDB,
            TransposeOrdering::BCAD => Self::CABD,
            TransposeOrdering::BCDA => Self::DABC,
            TransposeOrdering::BDAC => Self::CADB,
            TransposeOrdering::BDCA => Self::DACB,
            TransposeOrdering::CABD => Self::BCAD,
            TransposeOrdering::CADB => Self::BDAC,
            TransposeOrdering::CBDA => Self::DBAC,
            TransposeOrdering::CDBA => Self::DCAB,
            TransposeOrdering::DABC => Self::BCDA,
            TransposeOrdering::DACB => Self::BDCA,
            TransposeOrdering::DBAC => Self::CBDA,
            TransposeOrdering::DCAB => Self::CDBA,

            // All other orderings are self-inverting.
            _ => self,
        }
    }
}

impl From<u32> for TransposeOrdering {
    fn from(value: u32) -> Self {
        // There are 4! == 24 unique permutations of four elements, and values
        // above 23 use the `mod 24` equivalent.
        let value = value % 24;

        // The mapping from integer value to ordering is predictable (i.e.,
        // given the ordering specified by `n`, the ordering specified by
        // `n + 1` can be determined without reference to a table), but I can't
        // think of a better way of performing this in code.
        match value {
            0 => Self::ABCD,
            1 => Self::ABDC,
            2 => Self::ACBD,
            3 => Self::ACDB,
            4 => Self::ADBC,
            5 => Self::ADCB,
            6 => Self::BACD,
            7 => Self::BADC,
            8 => Self::BCAD,
            9 => Self::BCDA,
            10 => Self::BDAC,
            11 => Self::BDCA,
            12 => Self::CABD,
            13 => Self::CADB,
            14 => Self::CBAD,
            15 => Self::CBDA,
            16 => Self::CDAB,
            17 => Self::CDBA,
            18 => Self::DABC,
            19 => Self::DACB,
            20 => Self::DBAC,
            21 => Self::DBCA,
            22 => Self::DCAB,
            23 => Self::DCBA,

            // It shouldn't be possible to reach this condition, since
            // `0 <= n % 24 <= 23`.
            value => panic!("Invalid transpose ordering {value}. This is a library bug."),
        }
    }
}

/// Applies the Pokémon XOR cipher to the provided data, in place.
///
/// Callers are responsible for verifying that the input data has a length in
/// bytes which is divisible by 2.
///
/// All games from generation 4 and later use an XOR cipher on input treated as
/// 16-bit words, with key masks generated by advancing a linear congruential
/// pseudorandom number generator.
fn xor_mask(data: &mut [u8], initial_seed: u32) {
    if data.is_empty() {
        return;
    }

    assert_eq!(data.len() % 4, 0);

    let mut mask = initial_seed;
    let mut cursor = data;
    loop {
        // Advance the mask with each word.
        mask = rand(mask);

        // Pull the next word from the data. The crypto algorithm always uses
        // little endian ordering.
        let bytes = [cursor[0], cursor[1]];
        let word = u16::from_le_bytes(bytes);

        // Apply the mask.
        let word = word ^ (mask >> 16) as u16;

        // Replace the bytes in the data.
        let bytes = u16::to_le_bytes(word);
        (cursor[0], cursor[1]) = (bytes[0], bytes[1]);

        // Advance the cursor to the next word if we have more data.
        if cursor.len() > 2 {
            cursor = &mut cursor[2..];
        } else {
            break;
        }
    }
}

/// Produces a pseudorandom value using the linear congruential generator used
/// by generation 4 and later.
#[inline(always)]
fn rand(seed: u32) -> u32 {
    seed.wrapping_mul(0x41c6_4e6d).wrapping_add(0x0000_6073)
}
