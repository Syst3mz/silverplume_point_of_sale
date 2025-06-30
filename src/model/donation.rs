use crate::database::database_object::DatabaseObject;
use crate::database::object_mapper::ObjectMapper;
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::date_time_wrapper::WrapInDateTime;
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

impl DatabaseObject for Donation {
    fn build_object_mapper(&self) -> ObjectMapper {
        ObjectMapper::new("donations")
            .add_field("payment_method", self.payment_method.clone())
            .add_field("price", self.price)
    }
}

impl WrapInDateTime for Donation {}
impl Default for Donation {
    fn default() -> Donation {
        Self {
            payment_method: Default::default(),
            price: 0.0,
        }
    }
}