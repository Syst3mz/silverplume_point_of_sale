mod type_;

use iced::Element;
use iced::widget::{container, pick_list, row, text};
use iced_aw::number_input;
use serde::{Deserialize, Serialize};
use strum::VariantArray;
use crate::{HEADER_SIZE, RULE_HEIGHT};
use crate::admission::type_::Type_;
use crate::as_description::AsDescription;
use crate::as_transaction_record::AsTransactionRecord;
use crate::payment_method::PaymentMethod;
use crate::transaction_record::{TransactionKind, TransactionRecord};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Admission {
    kind: Option<Type_>,
    payment_method: Option<PaymentMethod>,
    quantity: u16,
}

#[derive(Debug, Clone)]
pub enum Message {
    KindSet(Type_),
    PaymentMethodSet(PaymentMethod),
    QuantitySet(u16),
}

impl Admission {
    fn needs_payment(&self) -> bool {
        let Some(admission_type) = self.kind.as_ref() else {
            return false;
        };
        
        !admission_type.is_free()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::KindSet(k) => self.kind = Some(k),
            Message::PaymentMethodSet(p) => self.payment_method = Some(p),
            Message::QuantitySet(q) => self.quantity = q,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut column = iced::widget::column![
            iced::widget::text("Admissions").size(HEADER_SIZE),
            iced::widget::horizontal_rule(RULE_HEIGHT),
            pick_list(Type_::VARIANTS, self.kind, Message::KindSet).placeholder("Select Admission Type"),
        ]
            .spacing(RULE_HEIGHT);
        
        
        if self.needs_payment() {
            column = column
                .push(pick_list(PaymentMethod::VARIANTS, self.payment_method, Message::PaymentMethodSet)
                .placeholder("Select Payment Method"));
        }
        
        container(
            column.push(row![text("Quantity: "), number_input(&self.quantity, 1..=u16::MAX, Message::QuantitySet)])
        ).into()
    }
    
    fn cost(&self) -> f32 {
        self.quantity as f32 * self.kind.map(|kind| kind.cost()).unwrap_or(0.0)
    }
}

impl Default for Admission {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            payment_method: None,
            quantity: 1,
        }
    }
}

impl AsTransactionRecord for Admission {
    fn as_transaction_record(&self) -> TransactionRecord {
        assert!(self.is_valid());
        TransactionRecord::new(
            TransactionKind::Admission, 
            self.kind.map(|x| x.as_description().to_string()).unwrap_or(String::from("ERROR: MISSING ADMISSION KIND")),
            self.quantity,
            self.cost()
        )
        
    }

    fn is_valid(&self) -> bool {
        let payment_ok = if self.needs_payment() { self.payment_method.is_some() } else { true };
        self.kind.is_some() && payment_ok && self.quantity > 0
    }
}