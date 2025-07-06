use sqlite::Row;
use crate::database::database_object::CanBuildObjectMapper;
use crate::database::from_sql::FromSql;
use crate::database::object_mapper::ObjectMapper;
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::date_time_wrapper::WrapInDateTime;
use crate::model::donation::Donation;
use crate::model::membership::kind::Kind;
use crate::model::payment_method::PaymentMethod;
use crate::model::transaction_record::{TransactionKind, TransactionRecord};

pub mod kind;
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Membership {
    kind: Kind,
    payment_method: PaymentMethod,
    pub quantity: u16
}


impl Membership {
    pub fn new(kind: Kind, payment_method: PaymentMethod, quantity: u16) -> Self {
        Self {
            kind,
            payment_method,
            quantity,
        }
    }
    pub fn matches_type(&self, kind: Kind) -> bool {
        self.kind == kind
    }
    
    pub fn compute_total_cost(&self) -> f32 {
        self.quantity as f32 * self.kind.price()
    }
}

impl AsTransactionRecord for Membership {
    fn as_transaction_record(&self) -> TransactionRecord {
        TransactionRecord::new (
            TransactionKind::Membership,
            self.kind.to_string(),
            self.quantity,
            self.compute_total_cost()
        )
    }
}

impl WrapInDateTime for Membership {}
impl CanBuildObjectMapper for Membership {
    const TABLE_NAME: &'static str = "memberships";

    fn build_object_mapper(&self) -> ObjectMapper {
        ObjectMapper::new(Self::TABLE_NAME)
            .add_field("kind", self.kind.to_string())
            .add_field("payment_method", self.payment_method.to_string())
            .add_field("quantity", self.quantity.to_string())
    }
}

impl FromSql for Membership {
    fn from_sql(row: Row) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let quantity:i64 = row.try_read("quantity")?;
        Ok(Self {
            kind: row.try_read("kind")?,
            payment_method: row.try_read("payment_method")?,
            quantity: quantity as u16,
        })
    }
}
impl Default for Membership {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            payment_method: Default::default(),
            quantity: 0,
        }
    }
}