use crate::model::payment_method::PaymentMethod;

pub trait HasPaymentMethod {
    fn payment_method(&self) -> Option<PaymentMethod>;
    fn matches_payment_method(&self, method: PaymentMethod) -> bool {
        let pm = self.payment_method();
        if pm.is_none() {
            return false;
        }

        pm == Some(method)
    }
}