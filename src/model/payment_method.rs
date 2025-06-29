use strum::{Display, VariantArray};

#[derive(Eq, PartialEq, Debug, Clone, Copy, VariantArray, Display, Default)]
pub enum PaymentMethod {
    #[default]
    Cash,
    CreditCard,
}