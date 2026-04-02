use std::io::Read;

use crate::errors::{BinError, ParseError};
use crate::{Storage, Transaction, TxStatus, TxType};

impl Storage {
    pub fn from_bin(reader: &mut impl std::io::Read) -> Result<Self, BinError> {
        todo!()
    }

    pub fn to_bin(&self, writer: &mut impl std::io::Write) -> Result<Self, BinError> {
        todo!()
    }
}

pub fn parse_bin_entity(entity: &[u8], entity_size: u32) -> Result<Transaction, ParseError> {
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
        return Err(ParseError::InvalidDescriptionEncoding);
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
