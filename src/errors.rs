#[derive(Debug)]
pub enum ParseError {
    WrongFieldCount(u8),
    InvalidTxType(String),
    InvalidTxStatus(String),
    WrongNumber(std::num::ParseIntError),
    WrongDelimeterFormat,
    WrongFieldName(String),
    MissingField(String),
    UnexpectedEOF(String),
    InvalidDescriptionEncoding,
    EntityTooSmallToBeValid(usize),
    HeaderTooSmallToBeValid(usize),
    WrongFormat(String),
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(from: std::num::ParseIntError) -> ParseError {
        ParseError::WrongNumber(from)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongFieldCount(n) => write!(f, "found {n} field instead of 8"),
            Self::InvalidTxType(t) => write!(
                f,
                "TX_TYPE is of the wrong format. Found {t} when only DEPOSIT, TRANSFER, WITHDRAWAL are allowed"
            ),
            Self::InvalidTxStatus(s) => write!(
                f,
                "TX_STATUS is of the wrong format. Found {s} when only SUCCESS, FAILURE, PENDING are allowed"
            ),
            Self::WrongNumber(err) => write!(f, "error parsing a number {err}"),
            Self::WrongDelimeterFormat => {
                write!(f, "error with parsing a text line with delimter as a cause")
            }
            Self::WrongFieldName(n) => write!(
                f,
                "error with parsing a transaction text block, encountered an unknown name {n}"
            ),
            Self::MissingField(n) => write!(f, "field {n} is missing"),
            Self::UnexpectedEOF(s) => write!(f, "while reading a binary, encountered {s}"),
            Self::InvalidDescriptionEncoding => write!(
                f,
                "while processing the binary description an error was encountered"
            ),
            Self::EntityTooSmallToBeValid(_) => {
                write!(f, "binary entity must be at least 46 bytes long")
            }
            Self::HeaderTooSmallToBeValid(_) => {
                write!(f, "binary header must be at least 8 bytes long")
            }
            Self::WrongFormat(s) => {
                write!(f, "unknown format type {s}")
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WrongNumber(err) => Some(err),
            _ => None,
        }
    }
}

/* ------------------------------------------------------------ */
#[derive(Debug)]
pub enum CSVError {
    Io(std::io::Error),
    InvalidHeader,
    Parse(ParseError),
}

impl From<std::io::Error> for CSVError {
    fn from(from: std::io::Error) -> CSVError {
        CSVError::Io(from)
    }
}

impl From<ParseError> for CSVError {
    fn from(from: ParseError) -> CSVError {
        CSVError::Parse(from)
    }
}

impl std::fmt::Display for CSVError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHeader => write!(f, "wrong csv header"),
            Self::Io(err) => write!(f, "error reading and writing {err}"),
            Self::Parse(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for CSVError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Parse(err) => Some(err),
            _ => None,
        }
    }
}

/* ------------------------------------------------------------ */
#[derive(Debug)]
pub enum BinError {
    Io(std::io::Error),
    Parse(ParseError),
    InvalidMagic(String),
}

impl From<std::io::Error> for BinError {
    fn from(from: std::io::Error) -> BinError {
        BinError::Io(from)
    }
}

impl From<ParseError> for BinError {
    fn from(from: ParseError) -> BinError {
        BinError::Parse(from)
    }
}

impl std::fmt::Display for BinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "error reading and writing {err}"),
            Self::Parse(err) => write!(f, "{err}"),
            Self::InvalidMagic(s) => write!(f, "one of the headers contains an invalid magic {s}"),
        }
    }
}

impl std::error::Error for BinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Parse(err) => Some(err),
            _ => None,
        }
    }
}

/* ------------------------------------------------------------ */
#[derive(Debug)]
pub enum TxtError {
    Io(std::io::Error),
    Parse(ParseError),
    DoubleSpaceBetweenEntities,
}

impl From<std::io::Error> for TxtError {
    fn from(from: std::io::Error) -> TxtError {
        TxtError::Io(from)
    }
}

impl From<ParseError> for TxtError {
    fn from(from: ParseError) -> TxtError {
        TxtError::Parse(from)
    }
}

impl std::fmt::Display for TxtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoubleSpaceBetweenEntities => {
                write!(f, "double space between transactions, only one is allowed")
            }
            Self::Io(err) => write!(f, "error reading and writing {err}"),
            Self::Parse(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for TxtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Parse(err) => Some(err),
            _ => None,
        }
    }
}
