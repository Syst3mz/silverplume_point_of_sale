use serde::{Deserialize, Serialize};
use strum::{Display, VariantArray};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, VariantArray, Display, Default)]
pub enum PaymentMethod {
    #[default]
    Cash,
    #[strum(serialize = "Credit Card")]
    CreditCard,
}