use iced::Element;
use iced::widget::{horizontal_rule, pick_list, text};
use strum::VariantArray;
use crate::as_transaction_record::AsTransactionRecord;
use crate::payment_method::PaymentMethod;
use crate::{HEADER_SIZE, RULE_HEIGHT};
use crate::decimal_input::DecimalInput;
use crate::get_payment_method::GetPaymentMethod;
use crate::transaction_record::{TransactionKind, TransactionRecord};

#[derive(Debug, Clone)]
pub struct Donation {
    pub payment_method: Option<PaymentMethod>,
    price: DecimalInput,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPaymentMethod(PaymentMethod),
    Price(crate::decimal_input::Message),
}

impl Donation {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetPaymentMethod(payment_method) => self.payment_method = Some(payment_method),
            Message::Price(p) => self.price.update(p),
        }
    }
    
    pub fn view(&self) -> Element<Message> {
        iced::widget::column![
            text("Donations").size(HEADER_SIZE),
            horizontal_rule(RULE_HEIGHT),
            pick_list(PaymentMethod::VARIANTS, self.payment_method, Message::SetPaymentMethod).placeholder("Select Payment Method"),
            self.price.view().map(Message::Price),
        ].spacing(RULE_HEIGHT).into()
    }
    
    pub fn amount(&self) -> f32 {
        self.price.value()
    }
}

impl Default for Donation {
    fn default() -> Self {
        Self {
            payment_method: Default::default(),
            price: DecimalInput::new("Amount", 0.0),
        }
    }
}

impl AsTransactionRecord for Donation {
    fn as_transaction_record(&self) -> TransactionRecord {
        assert!(self.is_valid());
        TransactionRecord::new(
            TransactionKind::Donation,
            "Donation".to_string(),
            1,
            self.price.value(),
        )
    }

    fn is_valid(&self) -> bool {
        self.payment_method.is_some()
    }
}

impl GetPaymentMethod for Donation {
    fn get_payment_method(&self) -> Option<PaymentMethod> {
        self.payment_method.clone()
    }
}