use std::hash::Hash;
use chrono::{DateTime, Local, TimeZone, Timelike};
use strum::Display;
use crate::database::database_object::DatabaseObject;
use crate::database::has_schema::{HasSchema, NOT_NULL};
use crate::database::object_mapper::ObjectMapper;
use crate::database::to_sql::ToSql;
use crate::model::date_time_wrapper::WrapInDateTime;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Default, Display)]
pub enum TransactionKind {
    #[default]
    Admission,
    Membership,
    Donation,
    #[strum(serialize = "Gift Shop Sales")]
    GiftShopSale,
}

impl HasSchema for TransactionKind {
    fn schema(field_name: &str) -> String
    where
        Self: Sized
    {
        format!("{} TEXT {}", field_name, NOT_NULL)
    }
}
impl ToSql for TransactionKind {
    fn to_sql(&self) -> String {
        format!("'{}'", self.to_string())
    }
}


#[derive(Debug, Clone)]
pub struct TransactionRecord {
    pub kind: TransactionKind,
    pub description: String,
    pub quantity: u16,
    pub total_cost: f32,
}

impl TransactionRecord {
    pub fn new(kind: TransactionKind, description: String, quantity: u16, amount: f32) -> Self {
        Self {
            kind,
            description,
            quantity,
            total_cost: amount,
        }
    }
}

impl DatabaseObject for TransactionRecord {
    fn build_object_mapper(&self) -> ObjectMapper {
        ObjectMapper::new("transaction_records")
            .add_field("kind", self.kind)
            .add_field("description", self.description.clone())
            .add_field("quantity", self.quantity as i32)
            .add_field("total_cost", self.total_cost)
    }
}
impl Default for TransactionRecord {
    fn default() -> Self {
        Self {
            kind: TransactionKind::Admission,
            description: "".to_string(),
            quantity: 0,
            total_cost: 0.0,
        }
    }
}
impl WrapInDateTime for TransactionRecord {}
