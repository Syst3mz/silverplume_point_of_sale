use anyhow::anyhow;
use iced::Element;
use iced::widget::{pick_list, row, text};
use iced_aw::number_input;
use strum::VariantArray;
use crate::{HEADER_SIZE, RULE_HEIGHT, TEXT_SIZE};
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::membership::kind::Kind;
use crate::model::payment_method::PaymentMethod;
use crate::model::transaction_record::{TransactionKind, TransactionRecord};
use crate::to_model::ToModel;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Membership {
    kind: Option<Kind>,
    payment_method: Option<PaymentMethod>,
    pub quantity: u16
}

#[derive(Debug, Clone)]
pub enum Message {
    Kind(Kind),
    PaymentMethod(PaymentMethod),
    Quantity(u16),
}
impl Membership {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Kind(k) => self.kind = Some(k),
            Message::PaymentMethod(p) => self.payment_method = Some(p),
            Message::Quantity(q) => self.quantity = q,
        }
    }

    pub fn view(&self) -> Element<Message> {
        iced::widget::column![
            iced::widget::text("Memberships").size(HEADER_SIZE),
            iced::widget::horizontal_rule(RULE_HEIGHT),
            pick_list(Kind::VARIANTS, self.kind, Message::Kind).placeholder("Select Membership Type"),
            pick_list(PaymentMethod::VARIANTS, self.payment_method, Message::PaymentMethod).placeholder("Select Payment Method"),
            row![text("Quantity: ").size(TEXT_SIZE), number_input(&self.quantity, 1..=u16::MAX, Message::Quantity,)].spacing(RULE_HEIGHT),
        ].spacing(RULE_HEIGHT).into()
    }

    pub fn compute_total_cost(&self) -> f32 {
        self.quantity as f32 * self.kind.map(|x| x.price()).unwrap_or(-1.0)
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.kind.is_some() && self.payment_method.is_some() && self.quantity >= 1
    }
}

impl Default for Membership {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            payment_method: Default::default(),
            quantity: 1,
        }
    }
}

impl AsTransactionRecord for Membership {
    fn as_transaction_record(&self) -> TransactionRecord {
        assert!(self.is_valid());

        TransactionRecord::new (
            TransactionKind::Membership,
            self.kind.map(|x| x.to_string())
                .unwrap_or(String::from("ERROR: MISSING MEMBERSHIP KIND")),
            self.quantity,
            self.compute_total_cost()
        )
    }
}

impl ToModel for Membership {
    type ModelType = crate::model::membership::Membership;
    fn to_model(&self) -> anyhow::Result<Self::ModelType> {
        Ok(Self::ModelType::new(
            self.kind.ok_or(anyhow!("membership kind is none, but cannot be!"))?,
            self.payment_method.ok_or(anyhow!("membership payment method is none, but cannot be!"))?,
            self.quantity))
    }
}