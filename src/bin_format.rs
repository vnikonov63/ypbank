use crate::{Transaction, TxStatus, TxType, Storage};
use crate::errors::BinError;

impl Storage { 
    pub fn from_bin<R: std::io::Read>(r: &mut R) -> Result<Self, BinError> {
        todo!()
    }

    pub fn to_bin<W: std::io::Write>(&self, writer: &mut W) -> Result<Self, BinError> {
        todo!()
    }
}