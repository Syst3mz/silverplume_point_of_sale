use strum::{Display, VariantArray};
use crate::database::has_schema::{HasSchema, NOT_NULL};
use crate::database::to_sql::ToSql;

#[derive(Eq, PartialEq, Debug, Clone, Copy, VariantArray, Display, Default)]
pub enum PaymentMethod {
    #[default]
    Cash,
    #[strum(serialize = "Credit Card")]
    CreditCard,
}

impl HasSchema for PaymentMethod {
    fn schema(field_name: &str) -> String
    where
        Self: Sized
    {
        format!("{field_name} TEXT {}", NOT_NULL)
    }
}

impl ToSql for PaymentMethod {
    fn to_sql(&self) -> String {
        format!("'{}'", self.to_string())
    }
}