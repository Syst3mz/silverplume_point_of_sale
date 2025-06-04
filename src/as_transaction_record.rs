use crate::transaction_record::TransactionRecord;

pub trait AsTransactionRecord {
    fn as_transaction_record(&self) -> TransactionRecord;
    fn is_valid(&self) -> bool;
}