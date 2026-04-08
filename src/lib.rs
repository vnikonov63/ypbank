use std::fs::File;
use std::io::BufReader;

use crate::errors::ParseError;

pub mod bin_format;
pub mod csv_format;
pub mod errors;
pub mod txt_format;

/// Financial Data format is represented here
///
/// ## Variants
///
/// * `Binary` - Binary Encoded Storage Format with a Header that contains information about the size of the block
/// * `Csv` - Comma-separated Storage Format
/// * `Txt` - Plain Text Storage Format, does not have any strict order, unlike other formats
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Format {
    /// Binary Encoded Storage Format
    Binary,
    /// Comma-separated Storage Format
    Csv,
    /// Plain Text Storage Format
    Txt,
}

/// Parses a string into a `Format`.
///
/// ## Examples
///
/// ```
/// use ypbank::Format;
/// use std::str::FromStr;
///
/// let f = Format::from_str("csv").unwrap();
/// assert_eq!(f, Format::Csv);
/// ```
///
/// ## Returns
///
/// Returns `ParseError::WrongFormat` if the string is not "binary", "csv", or "txt".
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

/// Reads a storage file from the given path and parses it into the `Storage` struct
/// Reads a storage file from the given path and parses it into a `Storage` struct.
///
/// The format of the file must be specified using the `Format` enum.
///
/// ## Arguments
///
/// * `path` - The path to the storage file.
/// * `format` - The format of the storage file (`Binary`, `Csv`, or `Txt`).
///
/// ## Returns
///
/// Returns `Ok(Storage)` if the file is successfully read and parsed, or an error boxed as `Box<dyn std::error::Error>`
/// if the file cannot be opened or parsed.
///
/// ## Examples
///
/// ```no_run
/// use ypbank::{read_storage, Format, Storage};
///
/// let storage = read_storage("transactions.csv", Format::Csv).unwrap();
/// assert!(!storage.transactions.is_empty());
/// ```
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

/// Storage, containing a Vec of Transactions is represented here
#[derive(Debug, PartialEq)]
pub struct Storage {
    pub transactions: Vec<Transaction>,
}

/// Transation, containing all of the necessary information about a financial transfer is represented here
///
/// ## Examples
///
/// ```
/// use ypbank::{Transaction, TxType, TxStatus};
///
/// let tx = Transaction {
///     tx_id: 1001,
///     tx_type: TxType::Deposit,
///     from_user_id: 0,
///     to_user_id: 501,
///     amount: 50000,
///     timestamp: 1672531200000,
///     status: TxStatus::Success,
///     description: "Initial account funding".to_string(),
/// };
/// ```
#[derive(Debug, PartialEq)]
pub struct Transaction {
    /// Unique transaction ID.
    pub tx_id: u64,
    /// Type of the Transaction, it can be either `Deposit` or `Transfer` or `Withdrawal`.
    pub tx_type: TxType,
    /// ID of the User who initiated the transaction. In case of a `Deposit` it is 0.
    pub from_user_id: u64,
    /// ID of the User who is the recepient of the transaction. In case of a `Withdrawal` it is 0.
    pub to_user_id: u64,
    /// Non-negative number representing the amount of money in the lowest-considered denomination (cents) in the transaction.
    pub amount: u64,
    /// Unix epoch timestamp in milliseconds saving the exact time the transaction was completed.
    pub timestamp: u64,
    /// Status of the Transaction, it can be either `Success` or `Pending` or `Failure`.
    pub status: TxStatus,
    /// Option description of the transaction.
    pub description: String,
}

/// Type of the Financial Transaction is represented here
///
/// ## Variants
///
/// * `Deposit` - Adding funds to the account the transaction is associated with, with `from_user_id` field initiated to 0
/// * `Transfer` - Movement of funds between `from_user_id` to `to_user_id`
/// * `Withdrawal` - Subtracting funds to the account the transaction is associated with, with `to_user_id` field initiated to 0
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TxType {
    /// Addition of funds
    Deposit,
    /// Movement of funds
    Transfer,
    /// Subtraction of funds
    Withdrawal,
}

/// Parses a string into a `TxType`.
///
/// ## Arguments
///
/// * `s` - A string slice representing the transaction type. Valid values are `"DEPOSIT"`, `"TRANSFER"`, `"WITHDRAWAL"`.
///
/// ## Returns
///
/// Returns `Ok(TxType)` if the string matches a valid transaction type, otherwise returns `Err(ParseError::InvalidTxType)`.
///
/// ## Examples
///
/// ```
/// use ypbank::{parse_tx_type, TxType};
///
/// let tx_type = parse_tx_type("DEPOSIT").unwrap();
/// assert_eq!(tx_type, TxType::Deposit);
/// ```
pub fn parse_tx_type(s: &str) -> Result<TxType, ParseError> {
    match s {
        "DEPOSIT" => Ok(TxType::Deposit),
        "TRANSFER" => Ok(TxType::Transfer),
        "WITHDRAWAL" => Ok(TxType::Withdrawal),
        _ => Err(ParseError::InvalidTxType(s.to_string())),
    }
}

impl std::fmt::Display for TxType {
    /// Formats a `TxType` as a string.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ypbank::{TxType};
    /// use std::fmt::Write;
    ///
    /// let tx_type = TxType::Transfer;
    /// let s = tx_type.to_string();
    /// assert_eq!(s, "TRANSFER");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TxType::Deposit => write!(f, "DEPOSIT"),
            TxType::Transfer => write!(f, "TRANSFER"),
            TxType::Withdrawal => write!(f, "WITHDRAWAL"),
        }
    }
}

/// Financial Data format is represented here
///
/// ## Variants
///
/// * `Binary` - Binary Encoded Storage Format with a Header that contains information about the size of the block
/// * `Csv` - Comma-separated Storage Format
/// * `Txt` - Plain Text Storage Format, does not have any strict order, unlike other formats
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TxStatus {
    Success,
    Failure,
    Pending,
}

impl std::fmt::Display for TxStatus {
    /// Formats a `TxStatus` as a string.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ypbank::{TxStatus};
    /// use std::fmt::Write;
    ///
    /// let tx_status = TxStatus::Success;
    /// let s = tx_status.to_string();
    /// assert_eq!(s, "SUCCESS");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TxStatus::Success => write!(f, "SUCCESS"),
            TxStatus::Failure => write!(f, "FAILURE"),
            TxStatus::Pending => write!(f, "PENDING"),
        }
    }
}

/// Parses a string into a `TxStatus`.
///
/// ## Arguments
///
/// * `s` - A string slice representing the transaction type. Valid values are `"SUCCESS"`, `"FAILURE"`, `"PENDING"`.
///
/// ## Returns
///
/// Returns `Ok(TxStatus)` if the string matches a valid transaction status, otherwise returns `Err(ParseError::InvalidTxStatus)`.
///
/// ## Examples
///
/// ```
/// use ypbank::{parse_tx_status, TxStatus};
///
/// let tx_status = parse_tx_status("SUCCESS").unwrap();
/// assert_eq!(tx_status, TxStatus::Success);
/// ```
pub fn parse_tx_status(s: &str) -> Result<TxStatus, ParseError> {
    match s {
        "SUCCESS" => Ok(TxStatus::Success),
        "FAILURE" => Ok(TxStatus::Failure),
        "PENDING" => Ok(TxStatus::Pending),
        _ => Err(ParseError::InvalidTxStatus(s.to_string())),
    }
}
