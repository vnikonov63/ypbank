use std::io::{BufRead, BufReader};

use crate::errors::{ParseError, TxtError};
use crate::{Storage, Transaction, TxStatus, TxType, parse_tx_status, parse_tx_type};

impl Storage {
    pub fn from_txt<R: std::io::Read>(reader: &mut R) -> Result<Self, TxtError> {
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
            }

            current_block.push(line_result.to_string());
        }

        Ok(Self { transactions })
    }

    pub fn to_txt<W: std::io::Write>(&self, writer: &mut W) -> Result<Self, TxtError> {
        todo!()
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
    use std::io::{Cursor, Read, Write};

    #[test]
    fn test_parse_text_line_correct() {
        let line = "TX_ID: 1234567890123456";
        let tx = parse_txt_line(&line).expect("Valid Line should pass");

        assert_eq!(tx.0, "TX_ID");
        assert_eq!(tx.1, "1234567890123456");
    }

    #[test]
    fn test_parse_text_line_no_delimeter() {
        let line = "TX_ID 1234567890123456";
        let tx = parse_txt_line(&line);

        assert!(matches!(tx, Err(ParseError::WrongDelimeterFormat)));
    }

    #[test]
    fn test_parse_text_line_too_many_delimeters() {
        let line = "TX_ID: 123456789:0123456";
        let tx = parse_txt_line(&line);

        assert!(matches!(tx, Err(ParseError::WrongDelimeterFormat)));
    }

    #[test]
    fn test_parse_txt_entity_correct() {
        let input: Vec<&str> = vec![
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
    fn test_from_txt_correct() {}

    #[test]
    fn test_to_txt_correct() {}
}
