use std::io::{BufReader, ErrorKind, Read};

use crate::errors::{BinError, ParseError};
use crate::{Storage, Transaction, TxStatus, TxType};

impl Storage {
    pub fn from_bin(reader: &mut impl std::io::Read) -> Result<Self, BinError> {
        let mut transactions = Vec::new();
        let mut entity = Vec::new();
        let mut f = BufReader::new(reader);

        loop {
            let mut header = [0u8; 8];
            match f.read_exact(&mut header) {
                Ok(()) => {}
                Err(err) => {
                    if err.kind() == ErrorKind::UnexpectedEof {
                        break;
                    }
                    return Err(BinError::Io(err.into()));
                }
            }

            let magic = &header[0..4];
            if magic != b"YPBN" {
                return Err(BinError::InvalidMagic(
                    magic.iter().map(|&b| b as char).collect::<String>(),
                ));
            }
            let entity_size_bin = &header[4..8];
            let entity_size = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);

            entity.resize(entity_size as usize, 0);
            f.read_exact(&mut entity)?;

            transactions.push(parse_bin_entity(&entity)?);
        }

        Ok(Self { transactions })
    }

    pub fn to_bin(&self, writer: &mut impl std::io::Write) -> Result<Self, BinError> {
        todo!()
    }
}

pub fn parse_bin_entity(entity: &[u8]) -> Result<Transaction, ParseError> {
    struct Offsets;
    impl Offsets {
        const TX_ID: usize = 0;
        const TX_TYPE: usize = 8;
        const FROM_USER_ID: usize = 9;
        const TO_USER_ID: usize = 17;
        const AMOUNT: usize = 25;
        const TIMESTAMP: usize = 33;
        const STATUS: usize = 41;
        const DESC_LEN: usize = 42;
        const DESC: usize = 46;
    }

    if entity.len() < 46 {
        return Err(ParseError::EntityTooSmallToBeValid(entity.len()));
    }

    let tx_id = read8bytes(entity, Offsets::TX_ID, "TX_ID")?;
    let tx_type = read1byte(entity, Offsets::TX_TYPE, "TX_TYPE")?;
    let tx_type = parse_fx_type_bin(tx_type)?;
    let from_user_id = read8bytes(entity, Offsets::FROM_USER_ID, "FROM_USER_ID")?;
    let to_user_id = read8bytes(entity, Offsets::TO_USER_ID, "TO_USER_ID")?;
    let amount = read8bytes(entity, Offsets::AMOUNT, "AMOUNT")?;
    let timestamp = read8bytes(entity, Offsets::TIMESTAMP, "TIMESTAMP")?;
    let status = read1byte(entity, Offsets::STATUS, "STATUS")?;
    let status = parse_fx_status_bin(status)?;
    let desc_len = read4bytes(entity, Offsets::DESC_LEN, "DESC_LEN")?;

    let desc_start = Offsets::DESC;
    let desc_end = desc_start + desc_len as usize;
    if entity.len() < desc_end as usize {
        return Err(ParseError::UnexpectedEOF("DESCRIPTION".to_string()));
    }

    let description_bytes = &entity[desc_start..desc_end];

    let description = match String::from_utf8(description_bytes.to_vec()) {
        Ok(s) => s,
        Err(_) => return Err(ParseError::WrongFieldName("DESCRIPTION".to_string())),
    };

    Ok(Transaction {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description,
    })
}

fn read1byte(data: &[u8], position: usize, field: &str) -> Result<u8, ParseError> {
    if data.len() < position + 1 {
        return Err(ParseError::UnexpectedEOF(field.to_string()));
    }
    let mut buffer = [0u8; 1];
    buffer.copy_from_slice(&data[position..position + 1]);
    Ok(u8::from_be_bytes(buffer))
}

fn read4bytes(data: &[u8], position: usize, field: &str) -> Result<u32, ParseError> {
    if data.len() < position + 4 {
        return Err(ParseError::UnexpectedEOF(field.to_string()));
    }
    let mut buffer = [0u8; 4];
    buffer.copy_from_slice(&data[position..position + 4]);
    Ok(u32::from_be_bytes(buffer))
}

fn read8bytes(data: &[u8], position: usize, field: &str) -> Result<u64, ParseError> {
    if data.len() < position + 8 {
        return Err(ParseError::UnexpectedEOF(field.to_string()));
    }
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&data[position..position + 8]);
    Ok(u64::from_be_bytes(buffer))
}

fn parse_fx_type_bin(s: u8) -> Result<TxType, ParseError> {
    match s {
        0 => Ok(TxType::Deposit),
        1 => Ok(TxType::Transfer),
        2 => Ok(TxType::Withdrawal),
        _ => Err(ParseError::InvalidTxType(s.to_string())),
    }
}

fn parse_fx_status_bin(s: u8) -> Result<TxStatus, ParseError> {
    match s {
        0 => Ok(TxStatus::Success),
        1 => Ok(TxStatus::Failure),
        2 => Ok(TxStatus::Pending),
        _ => Err(ParseError::InvalidTxStatus(s.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};

    #[test]
    fn test_parse_bin_entity_correct() {
        let mut entity = Vec::new();

        let tx_id: u64 = 1001;
        let tx_type: u8 = 0;
        let from_user_id: u64 = 0;
        let to_user_id: u64 = 501;
        let amount: u64 = 50000;
        let timestamp: u64 = 1672531200000;
        let status: u8 = 0;
        let description = "Initial account funding";
        let desc_len: u32 = description.len() as u32;

        entity.extend_from_slice(&tx_id.to_be_bytes());
        entity.push(tx_type);
        entity.extend_from_slice(&from_user_id.to_be_bytes());
        entity.extend_from_slice(&to_user_id.to_be_bytes());
        entity.extend_from_slice(&amount.to_be_bytes());
        entity.extend_from_slice(&timestamp.to_be_bytes());
        entity.push(status);
        entity.extend_from_slice(&desc_len.to_be_bytes());
        entity.extend_from_slice(description.as_bytes());

        let tx = parse_bin_entity(&entity).unwrap();

        assert_eq!(tx.tx_id, tx_id);
        assert_eq!(tx.tx_type, TxType::Deposit);
        assert_eq!(tx.from_user_id, from_user_id);
        assert_eq!(tx.to_user_id, to_user_id);
        assert_eq!(tx.amount, amount);
        assert_eq!(tx.timestamp, timestamp);
        assert_eq!(tx.status, TxStatus::Success);
        assert_eq!(tx.description, description.to_string());
    }

    /* #[test]
    fn test_parse_bin_entity_description_non_utf8() {
        todo!()
    }

    #[test]
    fn test_parse_bin_entity_too_small() {
        todo!()
    } */
}
