use iced::Element;
use iced::widget::{container, pick_list, row, text};
use iced_aw::number_input;
use strum::VariantArray;
use crate::{HEADER_SIZE, RULE_HEIGHT, TEXT_SIZE};
use crate::model::admission::kind::Kind;
use crate::model::payment_method::PaymentMethod;

#[derive(Debug, Clone, Copy)]
pub struct Admission {
    pub kind: Option<Kind>,
    payment_method: Option<PaymentMethod>,
    pub quantity: u16,
}

#[derive(Debug, Clone)]
pub enum Message {
    KindSet(Kind),
    PaymentMethodSet(PaymentMethod),
    QuantitySet(u16),
}

impl Admission {
    pub fn needs_payment(&self) -> bool {
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
            pick_list(Kind::VARIANTS, self.kind, Message::KindSet).placeholder("Select Admission Type"),
        ]
            .spacing(RULE_HEIGHT);


        if self.needs_payment() {
            column = column
                .push(pick_list(PaymentMethod::VARIANTS, self.payment_method, Message::PaymentMethodSet)
                    .placeholder("Select Payment Method"));
        }

        container(
            column.push(row![text("Quantity: ").size(TEXT_SIZE), number_input(&self.quantity, 1..=u16::MAX, Message::QuantitySet)].spacing(RULE_HEIGHT)),
        ).into()
    }

    pub fn matches_admission_type(&self, kind: Kind) -> bool {
        let Some(self_type) = self.kind.as_ref() else {
            return false;
        };

        *self_type == kind
    }

    pub fn compute_total_cost(&self) -> f32 {
        self.quantity as f32 * self.kind.map(|kind| kind.cost()).unwrap_or(0.0)
    }

    pub(crate) fn is_valid(&self) -> bool {
        let payment_ok = if self.needs_payment() { self.payment_method.is_some() } else { true };
        self.kind.is_some() && payment_ok && self.quantity > 0
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