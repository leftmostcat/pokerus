use core::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub const fn format_mismatch() -> Self {
        Error {
            kind: ErrorKind::FormatMismatch,
        }
    }

    pub const fn invalid_argument() -> Self {
        Error {
            kind: ErrorKind::InvalidArgument,
        }
    }

    pub const fn invalid_data_value() -> Self {
        Error {
            kind: ErrorKind::InvalidDataValue,
        }
    }

    pub fn is_format_mismatch(&self) -> bool {
        self.kind == ErrorKind::FormatMismatch
    }

    pub fn is_invalid_argument(&self) -> bool {
        self.kind == ErrorKind::InvalidArgument
    }

    pub fn is_invalid_data_value(&self) -> bool {
        self.kind == ErrorKind::InvalidDataValue
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ErrorKind {
    FormatMismatch,
    InvalidArgument,
    InvalidDataValue,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::FormatMismatch => write!(f, "data does not match specified format"),
            ErrorKind::InvalidArgument => write!(f, "function called with invalid argument"),
            ErrorKind::InvalidDataValue => {
                write!(f, "data contained value which could not be processed")
            }
        }
    }
}
