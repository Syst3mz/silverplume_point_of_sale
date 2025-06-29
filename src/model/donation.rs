use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::payment_method::PaymentMethod;
use crate::model::transaction_record::{TransactionKind, TransactionRecord};

#[derive(Debug, Clone)]
pub struct Donation {
    pub payment_method: PaymentMethod,
    pub price: f32,
}

impl AsTransactionRecord for Donation {
    fn as_transaction_record(&self) -> TransactionRecord {
        TransactionRecord::new(
            TransactionKind::Donation,
            "Donation".to_string(),
            1,
            self.price,
        )
    }
}