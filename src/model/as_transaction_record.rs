use crate::model::transaction_record::TransactionRecord;

pub trait AsTransactionRecord {
    fn as_transaction_record(&self) -> TransactionRecord;
}