use sqlite::Row;
use crate::database::database_object::CanBuildObjectMapper;
use crate::database::from_sql::FromSql;
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

impl CanBuildObjectMapper for Donation {
    const TABLE_NAME: &'static str = "donations";

    fn build_object_mapper(&self) -> ObjectMapper {
        ObjectMapper::new(Self::TABLE_NAME)
            .add_field("payment_method", self.payment_method.clone())
            .add_field("price", self.price)
    }
}

impl FromSql for Donation {
    fn from_sql(row: Row) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let price: f64 = row.try_read("price")?;
        Ok(Self {
            payment_method: row.try_read("payment_method")?,
            price: price as f32,
        })
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