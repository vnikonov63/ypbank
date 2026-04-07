use std::io::{BufRead, BufReader};

use crate::errors::{ParseError, TxtError};
use crate::{Storage, Transaction, TxStatus, TxType, parse_tx_status, parse_tx_type};

impl Storage {
    pub fn from_txt(reader: &mut impl std::io::Read) -> Result<Self, TxtError> {
        let mut transactions = Vec::new();
        let f = BufReader::new(reader);

        let mut current_block: Vec<String> = Vec::new();

        for line in f.lines() {
            let line_temp = line?;
            let line_result = line_temp.trim();

            if line_result.starts_with('#') {
                continue;
            }

            if line_result.is_empty() {
                if !current_block.is_empty() {
                    let entity = current_block.iter().map(|s| s.as_str()).collect();
                    transactions.push(parse_txt_entity(&entity)?);
                    current_block.clear();
                } else {
                    return Err(TxtError::DoubleSpaceBetweenEntities);
                }
                continue;
            }

            current_block.push(line_result.to_string());
        }

        if !current_block.is_empty() {
            let entity = current_block.iter().map(|s| s.as_str()).collect();
            transactions.push(parse_txt_entity(&entity)?);
        }

        Ok(Self { transactions })
    }

    pub fn to_txt(&self, writer: &mut impl std::io::Write) -> Result<(), TxtError> {
        let mut iter = self.transactions.iter().peekable();

        while let Some(tx) = iter.next() {
            writeln!(
                writer,
                "TX_ID: {}\nTX_TYPE: {}\nFROM_USER_ID: {}\nTO_USER_ID: {}\nAMOUNT: {}\nTIMESTAMP: {}\nSTATUS: {}\nDESCRIPTION: \"{}\"",
                tx.tx_id,
                tx.tx_type,
                tx.from_user_id,
                tx.to_user_id,
                tx.amount,
                tx.timestamp,
                tx.status,
                tx.description
            )?;

            if iter.peek().is_some() {
                writeln!(writer)?;
            }
        }

        writer.flush()?;
        Ok(())
    }
}

pub fn parse_txt_line(line: &str) -> Result<(&str, &str), ParseError> {
    let mut it = line.split(':');
    match (it.next(), it.next(), it.next()) {
        (Some(a), Some(b), None) if !a.is_empty() && !b.is_empty() => Ok((a.trim(), b.trim())),
        _ => Err(ParseError::WrongDelimeterFormat),
    }
}

pub fn parse_txt_entity(lines: &Vec<&str>) -> Result<Transaction, ParseError> {
    let mut tx_id: Option<u64> = None;
    let mut tx_type: Option<TxType> = None;
    let mut from_user_id: Option<u64> = None;
    let mut to_user_id: Option<u64> = None;
    let mut amount: Option<u64> = None;
    let mut timestamp: Option<u64> = None;
    let mut status: Option<TxStatus> = None;
    let mut description: Option<String> = None;

    for line in lines {
        if line.starts_with('#') {
            continue;
        }

        let (key, value) = parse_txt_line(line)?;

        match key {
            "TX_ID" => tx_id = Some(value.parse()?),
            "TX_TYPE" => tx_type = Some(parse_tx_type(value)?),
            "FROM_USER_ID" => from_user_id = Some(value.parse()?),
            "TO_USER_ID" => to_user_id = Some(value.parse()?),
            "AMOUNT" => amount = Some(value.parse()?),
            "TIMESTAMP" => timestamp = Some(value.parse()?),
            "STATUS" => status = Some(parse_tx_status(value)?),
            "DESCRIPTION" => description = Some(value.trim_matches('"').to_string()),
            _ => return Err(ParseError::WrongFieldName(key.to_string())),
        }
    }

    Ok(Transaction {
        tx_id: tx_id.ok_or(ParseError::MissingField("TX_ID".to_string()))?,
        tx_type: tx_type.ok_or(ParseError::MissingField("TX_TYPE".to_string()))?,
        from_user_id: from_user_id.ok_or(ParseError::MissingField("FROM_USER_ID".to_string()))?,
        to_user_id: to_user_id.ok_or(ParseError::MissingField("TO_USER_ID".to_string()))?,
        amount: amount.ok_or(ParseError::MissingField("AMOUNT".to_string()))?,
        timestamp: timestamp.ok_or(ParseError::MissingField("TIMESTAMP".to_string()))?,
        status: status.ok_or(ParseError::MissingField("STATUS".to_string()))?,
        description: description.ok_or(ParseError::MissingField("DESCRIPTION".to_string()))?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};

    #[test]
    fn test_parse_text_line_correct() {
        let line = "TX_ID: 1234567890123456";
        let tx = parse_txt_line(line).expect("Valid Line should pass");

        assert_eq!(tx.0, "TX_ID");
        assert_eq!(tx.1, "1234567890123456");
    }

    #[test]
    fn test_parse_text_line_no_delimeter() {
        let line = "TX_ID 1234567890123456";
        let tx = parse_txt_line(line);

        assert!(matches!(tx, Err(ParseError::WrongDelimeterFormat)));
    }

    #[test]
    fn test_parse_text_line_too_many_delimeters() {
        let line = "TX_ID: 123456789:0123456";
        let tx = parse_txt_line(line);

        assert!(matches!(tx, Err(ParseError::WrongDelimeterFormat)));
    }

    #[test]
    fn test_parse_txt_entity_correct() {
        let input: Vec<&str> = vec![
            "# Record 1 (Deposit)",
            "STATUS: SUCCESS",
            r#"DESCRIPTION: "Terminal deposit""#,
            "TO_USER_ID: 9876543210987654",
            "AMOUNT: 1000",
            "TIMESTAMP: 1633036800000",
            "TX_ID: 1234567890123456",
            "FROM_USER_ID: 0",
            "TX_TYPE: DEPOSIT",
        ];

        let tx = parse_txt_entity(&input).expect("Valid entity should be parsed");
        assert_eq!(tx.tx_id, 1234567890123456);
        assert_eq!(tx.tx_type, TxType::Deposit);
        assert_eq!(tx.from_user_id, 0);
        assert_eq!(tx.to_user_id, 9876543210987654);
        assert_eq!(tx.amount, 1000);
        assert_eq!(tx.timestamp, 1633036800000);
        assert_eq!(tx.status, TxStatus::Success);
        assert_eq!(tx.description, "Terminal deposit");
    }

    #[test]
    fn test_parse_txt_entity_wrong_filed_name() {
        let input: Vec<&str> = vec![
            "STATUS: SUCCESS",
            r#"DESCRIPTION: "Terminal deposit""#,
            "TO_USER_ID: 9876543210987654",
            "AMOUNT: 1000",
            "TIMESTAMP: 1633036800000",
            "TX_ID: 1234567890123456",
            "FROOOM_USER_ID: 0",
            "TX_TYPE: DEPOSIT",
        ];

        let tx = parse_txt_entity(&input);
        match tx {
            Err(ParseError::WrongFieldName(t)) => assert_eq!(t, "FROOOM_USER_ID"),
            _ => panic!("Expected WrongFieldName Error"),
        }
    }

    #[test]
    fn test_parse_txt_entity_missing_filed() {
        let input: Vec<&str> = vec![
            "STATUS: SUCCESS",
            r#"DESCRIPTION: "Terminal deposit""#,
            "TO_USER_ID: 9876543210987654",
            "TIMESTAMP: 1633036800000",
            "TX_ID: 1234567890123456",
            "FROM_USER_ID: 0",
            "TX_TYPE: DEPOSIT",
        ];

        let tx = parse_txt_entity(&input);
        match tx {
            Err(ParseError::MissingField(t)) => assert_eq!(t, "AMOUNT"),
            _ => panic!("Expected MissingField Error"),
        }
    }

    #[test]
    fn test_from_txt_correct() {
        let mut buffer = Cursor::new(Vec::new());

        let s = concat!(
            "# Record 1 (Deposit)\n",
            "TX_ID: 1234567890123456\n",
            "TX_TYPE: DEPOSIT\n",
            "FROM_USER_ID: 0\n",
            "TO_USER_ID: 9876543210987654\n",
            "AMOUNT: 10000\n",
            "TIMESTAMP: 1633036800000\n",
            "STATUS: SUCCESS\n",
            "DESCRIPTION: \"Terminal deposit\"\n",
            "\n",
            "# Record 2 (Transfer)\n",
            "TX_ID: 2312321321321321\n",
            "TIMESTAMP: 1633056800000\n",
            "STATUS: FAILURE\n",
            "TX_TYPE: TRANSFER\n",
            "FROM_USER_ID: 1231231231231231\n",
            "TO_USER_ID: 9876543210987654\n",
            "AMOUNT: 1000\n",
            "DESCRIPTION: \"User transfer\"\n",
            "\n",
            "# Record 3 (Withdrawal)\n",
            "TX_ID: 3213213213213213\n",
            "AMOUNT: 100\n",
            "TX_TYPE: WITHDRAWAL\n",
            "FROM_USER_ID: 9876543210987654\n",
            "TO_USER_ID: 0\n",
            "TIMESTAMP: 1633066800000\n",
            "STATUS: SUCCESS\n",
            "DESCRIPTION: \"User withdrawal\"\n",
        );

        write!(buffer, "{}", s).unwrap();

        buffer.set_position(0);

        let storage = Storage::from_txt(&mut buffer).expect("valid text should be read");

        assert_eq!(storage.transactions.len(), 3);

        let tx1 = &storage.transactions[0];
        assert_eq!(tx1.tx_id, 1234567890123456);
        assert_eq!(tx1.tx_type, TxType::Deposit);
        assert_eq!(tx1.from_user_id, 0);
        assert_eq!(tx1.to_user_id, 9876543210987654);
        assert_eq!(tx1.amount, 10000);
        assert_eq!(tx1.timestamp, 1633036800000);
        assert_eq!(tx1.status, TxStatus::Success);
        assert_eq!(tx1.description, "Terminal deposit");

        let tx2 = &storage.transactions[1];
        assert_eq!(tx2.tx_id, 2312321321321321);
        assert_eq!(tx2.tx_type, TxType::Transfer);
        assert_eq!(tx2.from_user_id, 1231231231231231);
        assert_eq!(tx2.to_user_id, 9876543210987654);
        assert_eq!(tx2.amount, 1000);
        assert_eq!(tx2.timestamp, 1633056800000);
        assert_eq!(tx2.status, TxStatus::Failure);
        assert_eq!(tx2.description, "User transfer");

        let tx3 = &storage.transactions[2];
        assert_eq!(tx3.tx_id, 3213213213213213);
        assert_eq!(tx3.tx_type, TxType::Withdrawal);
        assert_eq!(tx3.from_user_id, 9876543210987654);
        assert_eq!(tx3.to_user_id, 0);
        assert_eq!(tx3.amount, 100);
        assert_eq!(tx3.timestamp, 1633066800000);
        assert_eq!(tx3.status, TxStatus::Success);
        assert_eq!(tx3.description, "User withdrawal");
    }

    #[test]
    fn test_to_txt_correct() {
        let storage = Storage {
            transactions: vec![
                Transaction {
                    tx_id: 1234567890123456,
                    tx_type: TxType::Deposit,
                    from_user_id: 0,
                    to_user_id: 9876543210987654,
                    amount: 10000,
                    timestamp: 1633036800000,
                    status: TxStatus::Success,
                    description: "Terminal deposit".to_string(),
                },
                Transaction {
                    tx_id: 2312321321321321,
                    tx_type: TxType::Transfer,
                    from_user_id: 1231231231231231,
                    to_user_id: 9876543210987654,
                    amount: 1000,
                    timestamp: 1633056800000,
                    status: TxStatus::Failure,
                    description: "ATM withdrawal".to_string(),
                },
            ],
        };

        let mut buffer = Cursor::new(Vec::new());
        storage
            .to_txt(&mut buffer)
            .expect("valid CSV must be writen");

        let bytes = buffer.into_inner();
        let actual = String::from_utf8(bytes).expect("output must be valid utf-8");

        let expected = concat!(
            "TX_ID: 1234567890123456\n",
            "TX_TYPE: DEPOSIT\n",
            "FROM_USER_ID: 0\n",
            "TO_USER_ID: 9876543210987654\n",
            "AMOUNT: 10000\n",
            "TIMESTAMP: 1633036800000\n",
            "STATUS: SUCCESS\n",
            "DESCRIPTION: \"Terminal deposit\"\n",
            "\n",
            "TX_ID: 2312321321321321\n",
            "TX_TYPE: TRANSFER\n",
            "FROM_USER_ID: 1231231231231231\n",
            "TO_USER_ID: 9876543210987654\n",
            "AMOUNT: 1000\n",
            "TIMESTAMP: 1633056800000\n",
            "STATUS: FAILURE\n",
            "DESCRIPTION: \"ATM withdrawal\"\n",
        );

        assert_eq!(actual, expected);
    }
}
