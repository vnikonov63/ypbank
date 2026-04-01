#[derive(Debug)]
pub enum ParseError {
    WrongFieldCount(u8),
    InvalidTxType(String),
    InvalidTxStatus(String),
    WrongNumber(std::num::ParseIntError)
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
            Self::InvalidTxType(t) => write!(f, "TX_TYPE is of the wrong format. Found {t} when only DEPOSIT, TRANSFER, WITHDRAWAL are allowed"),
            Self::InvalidTxStatus(s) => write!(f, "TX_STATUS is of the wrong format. Found {s} when only SUCCESS, FAILURE, PENDING are allowed"),
            Self::WrongNumber(err) => write!(f, "error parsing a number {err}"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WrongNumber(err) => Some(err),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum CSVError {
    Io(std::io::Error),
    InvalidHeader(String),
    Parse(ParseError)
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
            Self::InvalidHeader(s) => write!(f, "wrong csv header"),
            Self::Io(err) => write!(f, "error reading and writing {err}"),
            Self::Parse(err) => write!(f, "{err}")
        }
    }
}

impl std::error::Error for CSVError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Parse(err) => Some(err),
            _ => None
        }
    }
}