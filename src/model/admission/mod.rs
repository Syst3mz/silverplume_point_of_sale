pub mod kind;

use sqlite::Row;
use crate::as_description::AsDescription;
use crate::database::database_object::CanBuildObjectMapper;
use crate::database::from_sql::{from_option, FromSql};
use crate::database::object_mapper::ObjectMapper;
use crate::model::admission::kind::Kind;
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::date_time_wrapper::WrapInDateTime;
use crate::model::has_payment_method::HasPaymentMethod;
use crate::model::has_total_cost::HasTotalCost;
use crate::model::payment_method::PaymentMethod;
use crate::model::transaction_record::{TransactionKind, TransactionRecord};

#[derive(Debug, Clone, Copy)]
pub struct Admission {
    pub kind: Kind,
    payment_method: Option<PaymentMethod>,
    pub quantity: u16,
}

impl Admission {
    pub fn new(kind: Kind, payment_method: Option<PaymentMethod>, quantity: u16) -> Admission {
        Self {
            kind,
            payment_method,
            quantity,
        }
    }
}

impl HasTotalCost for Admission {
    fn total_cost(&self) -> f32 {
        self.quantity as f32 * self.kind.cost()
    }
}

impl AsTransactionRecord for Admission {
    fn as_transaction_record(&self) -> TransactionRecord {
        TransactionRecord::new(
            TransactionKind::Admission,
            self.kind.as_description().to_string(),
            self.quantity,
            self.total_cost()
        )
    }
}

impl HasPaymentMethod for Admission {
    fn payment_method(&self) -> Option<PaymentMethod> {
        self.payment_method.clone()
    }
}

impl WrapInDateTime for Admission {}
impl CanBuildObjectMapper for Admission {
    const TABLE_NAME: &'static str = "admissions";

    fn build_object_mapper(&self) -> ObjectMapper {
        ObjectMapper::new(Self::TABLE_NAME)
            .add_field("kind", self.kind.to_string())
            .add_field("payment_method", self.payment_method)
            .add_field("quantity", self.quantity as i32)
    }
}
impl FromSql for Admission {
    fn from_sql(mut row: Row) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let x:i64 = row.try_read("quantity")?;
        Ok(Self {
            kind: row.try_read("kind")?,
            payment_method: from_option(&row.take("payment_method"))?,
            quantity: x as u16,
        })
    }
}
impl Default for Admission {
    fn default() -> Admission {
        Self {
            kind: Default::default(),
            payment_method: None,
            quantity: 0,
        }
    }
}