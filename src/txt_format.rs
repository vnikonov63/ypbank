use crate::errors::TxtError;
use crate::{Storage, Transaction, TxStatus, TxType};

impl Storage {
    pub fn from_txt<R: std::io::Read>(r: &mut R) -> Result<Self, TxtError> {
        todo!()
    }

    pub fn to_txt<W: std::io::Write>(&self, writer: &mut W) -> Result<Self, TxtError> {
        todo!()
    }
}
