use sqlite::Value;
use strum::{Display, EnumString, VariantArray};
use crate::database::has_schema::{HasSchema, NOT_NULL};
use crate::database::to_sql::ToSql;

#[derive(Eq, PartialEq, Debug, Clone, Copy, VariantArray, Display, Default, EnumString)]
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

impl TryFrom<&Value> for PaymentMethod {
    type Error = sqlite::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let Value::String(value) = value else {
            return Err(sqlite::Error {
                code: None,
                message: Some("Value is not a string, and must be.".to_string()),
            })
        };
        
        PaymentMethod::try_from(value.as_str()).map_err(|_| sqlite::Error {
            code: None,
            message: Some("Unable to convert string to payment method.".to_string()),
        })
    }
}