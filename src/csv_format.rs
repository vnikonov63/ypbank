use std::io::{BufRead, BufReader};

use crate::errors::{CSVError, ParseError};
use crate::{Storage, Transaction, parse_tx_status, parse_tx_type};

/// A Header and every csv file should start with one
/// Should be exactly: `TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION`
pub const CSV_HEADER: &str =
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

impl Storage {
    /// Reads a `Storage` from a CSV reader.
    ///
    /// The CSV format is expected to have a header line matching `CSV_HEADER` and one line per transaction.
    /// Empty lines are skipped, and the header is validated.
    ///
    /// ## Arguments
    ///
    /// * `reader` - Any type implementing `std::io::Read`.
    ///
    /// ## Returns
    ///
    /// Returns `Ok(Storage)` if all lines are successfully parsed.
    /// Returns `Err(CSVError)` if the header is invalid, a line is malformed, or any IO error occurs.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use std::io::Cursor;
    /// use ypbank::{Storage};
    /// use ypbank::csv_format::CSV_HEADER;
    ///
    /// let data = format!("{CSV_HEADER}\n1,DEPOSIT,0,100,5000,1672531200000,Success,\"Initial deposit\"");
    /// let mut reader = Cursor::new(data);
    /// let storage = Storage::from_csv(&mut reader).unwrap();
    /// assert_eq!(storage.transactions.len(), 1);
    /// ```
    pub fn from_csv(reader: &mut impl std::io::Read) -> Result<Self, CSVError> {
        let mut transactions = Vec::new();
        let f = BufReader::new(reader);

        for (i, line) in f.lines().enumerate() {
            let line_result = line?;
            if i == 0 {
                if line_result.trim_matches(|c| c == '\n' || c == '\r') != CSV_HEADER {
                    return Err(CSVError::InvalidHeader);
                }
                continue;
            }

            if line_result.trim().is_empty() {
                continue;
            }

            let tx = parse_csv_line(&line_result)?;
            transactions.push(tx);
        }

        Ok(Self { transactions })
    }

    /// Writes `Storage` to a CSV writer.
    ///
    /// The first line is the CSV header (`CSV_HEADER`), followed by one line per transaction.
    ///
    /// ## Arguments
    ///
    /// * `writer` - Any type implementing `std::io::Write`.
    ///
    /// ## Returns
    ///
    /// Returns `Ok(())` if the storage is successfully serialized.
    /// Returns `Err(CSVError)` if any IO error occurs during writing.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use std::io::Cursor;
    /// use ypbank::{Storage, TxType, Transaction, TxStatus};
    ///
    /// let storage = Storage { transactions: vec![] };
    /// let mut writer = Cursor::new(Vec::new());
    /// storage.to_csv(&mut writer).unwrap();
    /// ```
    pub fn to_csv<W: std::io::Write>(&self, writer: &mut W) -> Result<(), CSVError> {
        writeln!(writer, "{CSV_HEADER}")?;
        for tx in &self.transactions {
            writeln!(
                writer,
                "{},{},{},{},{},{},{},\"{}\"",
                tx.tx_id,
                tx.tx_type,
                tx.from_user_id,
                tx.to_user_id,
                tx.amount,
                tx.timestamp,
                tx.status,
                tx.description
            )?;
        }
        writer.flush()?;
        Ok(())
    }
}

fn parse_csv_line(line: &str) -> Result<Transaction, ParseError> {
    let bits: Vec<&str> = line.split(',').collect();
    if bits.len() != 8 {
        return Err(ParseError::WrongFieldCount(bits.len() as u8));
    }

    let transaction = Transaction {
        tx_id: bits[0].parse()?,
        tx_type: parse_tx_type(bits[1])?,
        from_user_id: bits[2].parse()?,
        to_user_id: bits[3].parse()?,
        amount: bits[4].parse()?,
        timestamp: bits[5].parse()?,
        status: parse_tx_status(bits[6])?,
        description: bits[7].trim_matches('"').to_string(),
    };

    Ok(transaction)
}

#[cfg(test)]
mod csv_format_tests {

    use super::*;
    use crate::{TxStatus, TxType};
    use std::error::Error;
    use std::io::{Cursor, Write};

    #[test]
    fn test_parse_one_csv_line_correct() -> Result<(), Box<dyn Error>> {
        let line = r#"1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#;
        let tx = parse_csv_line(line)?;

        assert_eq!(tx.tx_id, 1001);
        assert_eq!(tx.tx_type, TxType::Deposit);
        assert_eq!(tx.from_user_id, 0);
        assert_eq!(tx.to_user_id, 501);
        assert_eq!(tx.amount, 50000);
        assert_eq!(tx.timestamp, 1672531200000);
        assert_eq!(tx.status, TxStatus::Success);
        assert_eq!(tx.description, "Initial account funding");

        Ok(())
    }

    #[test]
    fn test_parse_one_csv_line_tx_type_invalid() {
        let line = r#"1001,INVALID,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#;
        let tx = parse_csv_line(line);

        match tx {
            Err(ParseError::InvalidTxType(t)) => assert_eq!(t, "INVALID"),
            _ => panic!("Expected InvalidTxType Error"),
        }
    }

    #[test]
    fn test_parse_one_csv_line_tx_status_invalid() {
        let line = r#"1001,DEPOSIT,0,501,50000,1672531200000,INVALID,"Initial account funding""#;
        let tx = parse_csv_line(line);

        match tx {
            Err(ParseError::InvalidTxStatus(t)) => assert_eq!(t, "INVALID"),
            _ => panic!("Expected InvalidTxStatus Error"),
        }
    }

    #[test]
    fn test_parse_one_csv_line_number_of_fields_invalid() {
        let line = "1001,DEPOSIT,0";
        let tx = parse_csv_line(line);

        match tx {
            Err(ParseError::WrongFieldCount(n)) => assert_eq!(n, 3),
            _ => panic!("Expected WrongFieldCount Error"),
        }
    }

    #[test]
    fn test_parse_one_csv_line_parse_int_error() {
        let line = r#"abc,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#;

        let tx = parse_csv_line(line);

        assert!(matches!(tx, Err(ParseError::WrongNumber(_))));
    }

    #[test]
    fn test_from_csv_wrong_header() -> Result<(), Box<dyn Error>> {
        let mut buffer = Cursor::new(Vec::new());
        writeln!(buffer, "TX_ID,TX_TYPE,FROM_USER_ID")?;
        writeln!(
            buffer,
            r#"1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#
        )?;

        buffer.set_position(0);

        let storage = Storage::from_csv(&mut buffer);

        assert!(matches!(storage, Err(CSVError::InvalidHeader)));

        Ok(())
    }

    #[test]
    fn test_from_csv_correct() -> Result<(), Box<dyn Error>> {
        let mut buffer = Cursor::new(Vec::new());
        writeln!(
            buffer,
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
        )?;
        writeln!(
            buffer,
            r#"1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#
        )?;
        writeln!(
            buffer,
            r#"1003,WITHDRAWAL,502,0,1000,1672538400000,PENDING,"ATM withdrawal""#
        )?;

        buffer.set_position(0);

        let storage = Storage::from_csv(&mut buffer)?;

        assert_eq!(storage.transactions.len(), 2);

        let tx1 = &storage.transactions[0];
        assert_eq!(tx1.tx_id, 1001);
        assert_eq!(tx1.tx_type, TxType::Deposit);
        assert_eq!(tx1.from_user_id, 0);
        assert_eq!(tx1.to_user_id, 501);
        assert_eq!(tx1.amount, 50000);
        assert_eq!(tx1.timestamp, 1672531200000);
        assert_eq!(tx1.status, TxStatus::Success);
        assert_eq!(tx1.description, "Initial account funding");

        let tx2 = &storage.transactions[1];
        assert_eq!(tx2.tx_id, 1003);
        assert_eq!(tx2.tx_type, TxType::Withdrawal);
        assert_eq!(tx2.from_user_id, 502);
        assert_eq!(tx2.to_user_id, 0);
        assert_eq!(tx2.amount, 1000);
        assert_eq!(tx2.timestamp, 1672538400000);
        assert_eq!(tx2.status, TxStatus::Pending);
        assert_eq!(tx2.description, "ATM withdrawal");

        Ok(())
    }

    #[test]
    fn test_to_csv_correct() -> Result<(), Box<dyn Error>> {
        let storage = Storage {
            transactions: vec![
                Transaction {
                    tx_id: 1001,
                    tx_type: TxType::Deposit,
                    from_user_id: 0,
                    to_user_id: 501,
                    amount: 50000,
                    timestamp: 1672531200000,
                    status: TxStatus::Success,
                    description: "Initial account funding".to_string(),
                },
                Transaction {
                    tx_id: 1003,
                    tx_type: TxType::Withdrawal,
                    from_user_id: 502,
                    to_user_id: 0,
                    amount: 1000,
                    timestamp: 1672538400000,
                    status: TxStatus::Pending,
                    description: "ATM withdrawal".to_string(),
                },
            ],
        };

        let mut buffer = Cursor::new(Vec::new());
        storage.to_csv(&mut buffer)?;

        let bytes = buffer.into_inner();
        let actual = String::from_utf8(bytes)?;

        let expected = concat!(
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n",
            "1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,\"Initial account funding\"\n",
            "1003,WITHDRAWAL,502,0,1000,1672538400000,PENDING,\"ATM withdrawal\"\n"
        );

        assert_eq!(actual, expected);

        Ok(())
    }
}
