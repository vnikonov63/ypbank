use crate::errors::{TxtError, ParseError};
use crate::{Storage, Transaction, TxStatus, TxType, parse_tx_status, parse_tx_type};

impl Storage {
    pub fn from_txt<R: std::io::Read>(r: &mut R) -> Result<Self, TxtError> {
        todo!()
    }

    pub fn to_txt<W: std::io::Write>(&self, writer: &mut W) -> Result<Self, TxtError> {
        todo!()
    }
}

pub fn parse_txt_line(line: &str) -> Result<(&str, &str), ParseError> {
    let mut it = line.split(':');
    match (it.next(), it.next(), it.next()) {
        (Some(a), Some(b), None) if !a.is_empty() && !b.is_empty() => Ok((a, b)),
        _ => Err(ParseError::WrongDelimeterFormat),
    }
}

pub fn parse_txt_entity(lines: &[String]) -> Result<Transaction, ParseError> {
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
        let key = key.trim();
        let value = value.trim();

        match key {
            "TX_ID" => {
                tx_id = Some(value.parse()?)
            },
            "TX_TYPE" => {
                tx_type = Some(parse_tx_type(value)?)
            },
            "FROM_USER_ID" => {
                from_user_id = Some(value.parse()?)
            },
            "TO_USER_ID" => {
                to_user_id = Some(value.parse()?)
            },
            "AMOUNT" => {
                amount = Some(value.parse()?)
            },
            "TIMESTAMP" => {
                timestamp = Some(value.parse()?)
            },
            "STATUS" => {
                status = Some(parse_tx_status(value)?)
            },
            "DESCRIPTION" => {
                description = Some(value.trim_matches('"').to_string())
            },
            _ => {
                return Err(ParseError::WrongFieldName(line.to_string()))
            }
        }
    }

    Ok(Transaction{
        tx_id: tx_id.ok_or(ParseError::MissingField("TX_ID".to_string()))?,
        tx_type: tx_type.ok_or(ParseError::MissingField("TX_TYPE".to_string()))?,
        from_user_id: from_user_id.ok_or(ParseError::MissingField("FROM_USER_ID".to_string()))?,
        to_user_id: to_user_id.ok_or(ParseError::MissingField("TO_USER_ID".to_string()))?,
        amount: amount.ok_or(ParseError::MissingField("AMOUNT".to_string()))?,
        timestamp: timestamp.ok_or(ParseError::MissingField("TIMESTAMP".to_string()))?,
        status: status.ok_or(ParseError::MissingField("STATUS".to_string()))?,
        description: description.ok_or(ParseError::MissingField("DESCRIPTION".to_string()))?
    })
}