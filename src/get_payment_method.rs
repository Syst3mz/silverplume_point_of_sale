use crate::payment_method::PaymentMethod;

pub trait GetPaymentMethod {
    fn get_payment_method(&self) -> Option<PaymentMethod>;
}