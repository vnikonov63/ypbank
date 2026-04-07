use std::io::{BufRead, BufReader};

use crate::errors::{CSVError, ParseError};
use crate::{Storage, Transaction, parse_tx_status, parse_tx_type};

const CSV_HEADER: &str =
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

impl Storage {
    /* --------!!!IMPORTANT MEMORY MOMENT!!!----------
    // this could be writen as
    pub fn from_csv<R: std::io::Read>(reader: &mut R) -> Result<Self, CSVError>
    // with Trait Bound Syntx and also using syntactic sugar as below */

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

pub fn parse_csv_line(line: &str) -> Result<Transaction, ParseError> {
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
mod tests {

    use super::*;
    use crate::{TxStatus, TxType};
    use std::io::{Cursor, Write};

    #[test]
    fn test_parse_one_csv_line_correct() {
        let line = r#"1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#;
        let tx = parse_csv_line(line).expect("Valid CSV should pass");

        assert_eq!(tx.tx_id, 1001);
        assert_eq!(tx.tx_type, TxType::Deposit);
        assert_eq!(tx.from_user_id, 0);
        assert_eq!(tx.to_user_id, 501);
        assert_eq!(tx.amount, 50000);
        assert_eq!(tx.timestamp, 1672531200000);
        assert_eq!(tx.status, TxStatus::Success);
        assert_eq!(tx.description, "Initial account funding");
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
    fn test_from_csv_wrong_header() {
        let mut buffer = Cursor::new(Vec::new());
        writeln!(buffer, "TX_ID,TX_TYPE,FROM_USER_ID")
            .expect("Should be able to write to a Cursor virtual stream");
        writeln!(
            buffer,
            r#"1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#
        )
        .unwrap();

        buffer.set_position(0);

        let storage = Storage::from_csv(&mut buffer);

        assert!(matches!(storage, Err(CSVError::InvalidHeader)));
    }

    #[test]
    fn test_from_csv_correct() {
        let mut buffer = Cursor::new(Vec::new());
        writeln!(
            buffer,
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
        )
        .expect("Should be able to write to a Cursor virtual stream");
        writeln!(
            buffer,
            r#"1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#
        )
        .unwrap();
        writeln!(
            buffer,
            r#"1003,WITHDRAWAL,502,0,1000,1672538400000,PENDING,"ATM withdrawal""#
        )
        .unwrap();

        buffer.set_position(0);

        let storage = Storage::from_csv(&mut buffer).expect("valid CSV must be read");

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
    }

    #[test]
    fn test_to_csv_correct() {
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
        storage
            .to_csv(&mut buffer)
            .expect("valid CSV must be writen");

        let bytes = buffer.into_inner();
        let actual = String::from_utf8(bytes).expect("output must be valid utf-8");

        let expected = concat!(
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n",
            "1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,\"Initial account funding\"\n",
            "1003,WITHDRAWAL,502,0,1000,1672538400000,PENDING,\"ATM withdrawal\"\n"
        );

        assert_eq!(actual, expected);
    }
}
