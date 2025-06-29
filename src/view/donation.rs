use iced::Element;
use crate::decimal_input::DecimalInput;
use crate::model::payment_method::PaymentMethod;
use iced::widget::{horizontal_rule, pick_list, text};
use strum::VariantArray;
use crate::{HEADER_SIZE, RULE_HEIGHT};

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

    pub(crate) fn is_valid(&self) -> bool {
        self.payment_method.is_some()
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