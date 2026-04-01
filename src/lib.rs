pub mod errors;
pub mod csv_format;
pub mod bin_format;
pub mod txt_format;
pub struct Storage {
    pub transactions: Vec<Transaction>
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal
}

#[derive(Debug, PartialEq)]
pub enum TxStatus {
    Success,
    Failure,
    Pending,
}
