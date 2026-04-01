use crate::errors::{CSVError, ParseError};
use crate::{Transaction, TxType, TxStatus, Storage};

impl Storage { 
    pub fn from_csv<R: std::io::Read>(r: &mut R) -> Result<Self, CSVError> {
        todo!()
    }

    pub fn to_csv<W: std::io::Write>(&self, writer: &mut W) -> Result<Self, CSVError> {
        todo!()
    }
}

pub fn parse_csv_line(line: &str) -> Result<Transaction, ParseError> {
    let bits: Vec<&str> = line.split(',').collect();
    if bits.len() != 8 {
        return Err(ParseError::WrongFieldCount(bits.len() as u8))
    }

    let transaction = Transaction {
        tx_id: bits[0].parse()?,
        tx_type: parse_tx_type(bits[1])?,
        from_user_id: bits[2].parse()?,
        to_user_id: bits[3].parse()?,
        amount: bits[4].parse()?,
        timestamp: bits[5].parse()?,
        status: parse_tx_status(bits[6])?,
        description: bits[7].trim_matches('"').to_string()
    };

    Ok(transaction)
}

pub fn parse_tx_type(s: &str) -> Result<TxType, ParseError> {
    match s {
        "DEPOSIT" => Ok(TxType::Deposit),
        "TRANSFER" => Ok(TxType::Transfer),
        "WITHDRAWAL" => Ok(TxType::Withdrawal),
        _ => Err(ParseError::InvalidTxType(s.to_string())),
    }
}

pub fn parse_tx_status(s: &str) -> Result<TxStatus, ParseError> {
    match s {
        "SUCCESS" => Ok(TxStatus::Success),
        "FAILURE" => Ok(TxStatus::Failure),
        "PENDING" => Ok(TxStatus::Pending),
        _ => Err(ParseError::InvalidTxStatus(s.to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_one_csv_line_correct() {
        let line = r#"1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#;
        let tx = parse_csv_line(&line).expect("Valid CSV should pass");

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
        let tx = parse_csv_line(&line);

        match tx {
            Err(ParseError::InvalidTxType(t)) => assert_eq!(t, "INVALID"),
            _ => panic!("Expected InvalidTxType Error")
        }
    }

    #[test]
    fn test_parse_one_csv_line_tx_status_invalid() {
        let line = r#"1001,DEPOSIT,0,501,50000,1672531200000,INVALID,"Initial account funding""#;
        let tx = parse_csv_line(&line);

        match tx {
            Err(ParseError::InvalidTxStatus(t)) => assert_eq!(t, "INVALID"),
            _ => panic!("Expected InvalidTxStatus Error")
        }
    }

    #[test]
    fn test_parse_one_csv_line_number_of_fields_invalid() {
        let line = "1001,DEPOSIT,0";
        let tx = parse_csv_line(&line);

        match tx {
            Err(ParseError::WrongFieldCount(n)) => assert_eq!(n, 3),
            _ => panic!("Expected WrongFieldCount Error")
        }
    }

    #[test]
    fn test_parse_one_csv_line_parse_int_error() {
        let line = r#"abc,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding""#;

        let tx = parse_csv_line(line);

        assert!(matches!(tx, Err(ParseError::WrongNumber(_))));
    }

}