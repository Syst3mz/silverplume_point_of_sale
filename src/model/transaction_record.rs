use std::hash::Hash;
use sqlite::{Row, Value};
use strum::{Display, EnumString};
use crate::database::database_object::CanBuildObjectMapper;
use crate::database::from_sql::FromSql;
use crate::database::has_schema::{HasSchema, NOT_NULL};
use crate::database::object_mapper::ObjectMapper;
use crate::database::to_sql::ToSql;
use crate::model::date_time_wrapper::WrapInDateTime;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Default, Display, EnumString)]
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
impl TryFrom<&Value> for TransactionKind {
    type Error = sqlite::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let Value::String(value) = value else {
            return Err(sqlite::Error {
                code: None,
                message: Some("Value is not a string, and must be.".to_string()),
            })
        };

        TransactionKind::try_from(value.as_str()).map_err(|_| sqlite::Error {
            code: None,
            message: Some("Unable to convert string to transaction kind.".to_string()),
        })
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

impl CanBuildObjectMapper for TransactionRecord {
    const TABLE_NAME: &'static str = "transaction_records";

    fn build_object_mapper(&self) -> ObjectMapper {
        ObjectMapper::new(Self::TABLE_NAME)
            .add_field("kind", self.kind)
            .add_field("description", self.description.clone())
            .add_field("quantity", self.quantity as i32)
            .add_field("total_cost", self.total_cost)
    }
}

impl FromSql for TransactionRecord {
    fn from_sql(row: Row) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let quantity:i64 = row.try_read("quantity")?;
        let total_cost:f64 = row.try_read("total_cost")?;
        Ok(Self {
            kind: row.try_read("kind")?,
            description: row.try_read::<&str, _>("description")?.to_string(),
            quantity: quantity as u16,
            total_cost: total_cost as f32,
        })
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
