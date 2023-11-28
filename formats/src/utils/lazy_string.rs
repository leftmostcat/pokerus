use core::cell::OnceCell;

/// `LazyString` stores a non-UTF-8 string as raw bytes, decoding to UTF-8 on
/// demand and storing the result to prevent duplicate processing.
///
/// A given string is tied to a single source encoding for simplicity and with
/// the expectation that
pub struct LazyString<'a, E> {
    raw: &'a [u8],
    cell: OnceCell<Result<String, E>>,
}

impl<'a, E> LazyString<'a, E> {
    /// Gets the result of trying to decode the stored bytes or tries decoding.
    /// Returns an error if the string cannot be decoded with the provided
    /// codec.
    pub fn get_or_try_decode<Dec>(&self, codec: &Dec) -> Result<&str, &E>
    where
        Dec: StringCodec<Error = E>,
    {
        self.cell
            .get_or_init(|| codec.try_decode(self.raw))
            .as_ref()
            .map(|string| string.as_str())
    }
}

/// A trait for a means to convert between strings in a given encoding and
/// strings in UTF-8.
pub trait StringCodec {
    type Error;

    /// Decodes a string from the encoding supported by the codec into UTF-8.
    fn try_decode(&self, value: &[u8]) -> Result<String, Self::Error>;

    /// Encodes a UTF-8 string into the encoding supported by the codec.
    fn try_encode(&self, value: &str) -> Result<Vec<u8>, Self::Error>;
}

impl<'a, E> From<&'a [u8]> for LazyString<'a, E> {
    fn from(value: &'a [u8]) -> Self {
        Self {
            raw: value,
            cell: OnceCell::new(),
        }
    }
}
