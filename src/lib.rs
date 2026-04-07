use std::fs::File;
use std::io::BufReader;

use crate::errors::ParseError;

pub mod bin_format;
pub mod csv_format;
pub mod errors;
pub mod txt_format;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Format {
    Binary,
    Csv,
    Txt,
}

impl std::str::FromStr for Format {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "binary" => Ok(Format::Binary),
            "csv" => Ok(Format::Csv),
            "txt" => Ok(Format::Txt),
            _ => Err(ParseError::WrongFormat(s.to_string())),
        }
    }
}

pub fn read_storage(path: &str, format: Format) -> Result<Storage, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let storage = match format {
        Format::Binary => Storage::from_bin(&mut reader)?,
        Format::Csv => Storage::from_csv(&mut reader)?,
        Format::Txt => Storage::from_txt(&mut reader)?,
    };

    Ok(storage)
}

#[derive(Debug, PartialEq)]
pub struct Storage {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TxType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: TxStatus,
    pub description: String,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

pub fn parse_tx_type(s: &str) -> Result<TxType, ParseError> {
    match s {
        "DEPOSIT" => Ok(TxType::Deposit),
        "TRANSFER" => Ok(TxType::Transfer),
        "WITHDRAWAL" => Ok(TxType::Withdrawal),
        _ => Err(ParseError::InvalidTxType(s.to_string())),
    }
}

impl std::fmt::Display for TxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TxType::Deposit => write!(f, "DEPOSIT"),
            TxType::Transfer => write!(f, "TRANSFER"),
            TxType::Withdrawal => write!(f, "WITHDRAWAL"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TxStatus {
    Success,
    Failure,
    Pending,
}

impl std::fmt::Display for TxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TxStatus::Success => write!(f, "SUCCESS"),
            TxStatus::Failure => write!(f, "FAILURE"),
            TxStatus::Pending => write!(f, "PENDING"),
        }
    }
}

pub fn parse_tx_status(s: &str) -> Result<TxStatus, ParseError> {
    match s {
        "SUCCESS" => Ok(TxStatus::Success),
        "FAILURE" => Ok(TxStatus::Failure),
        "PENDING" => Ok(TxStatus::Pending),
        _ => Err(ParseError::InvalidTxStatus(s.to_string())),
    }
}
