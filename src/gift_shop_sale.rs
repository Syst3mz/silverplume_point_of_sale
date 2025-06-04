use iced::Element;
use iced::widget::{pick_list, text_input};
use iced_aw::number_input;
use serde::{Deserialize, Serialize};
use strum::VariantArray;
use crate::payment_method::PaymentMethod;
use crate::decimal_input::DecimalInput;
use crate::{HEADER_SIZE, RULE_HEIGHT};
use crate::as_transaction_record::AsTransactionRecord;
use crate::get_payment_method::GetPaymentMethod;
use crate::transaction_record::{TransactionKind, TransactionRecord};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GiftShopSale {
    item_description: String,
    price: DecimalInput,
    pub payment_method: Option<PaymentMethod>,
    quantity: u16,
    sales_tax: DecimalInput
}

const DEFAULT_SALES_TAX: f32 = 8.55 ;
impl Default for GiftShopSale {
    fn default() -> Self {
        Self {
            item_description: Default::default(),
            price: DecimalInput::new("Item Price", 0.0),
            payment_method: Default::default(),
            quantity: 1,
            sales_tax: DecimalInput::new("Sales Tax", DEFAULT_SALES_TAX),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    DescriptionChanged(String),
    PriceChanged(crate::decimal_input::Message),
    PaymentMethodChanged(PaymentMethod),
    QuantityChanged(u16),
    SalesTaxChanged(crate::decimal_input::Message),
}
impl GiftShopSale {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::DescriptionChanged(s) => self.item_description = s,
            Message::PriceChanged(p) => self.price.update(p),
            Message::PaymentMethodChanged(m) => self.payment_method = Some(m),
            Message::QuantityChanged(q) => self.quantity = q,
            Message::SalesTaxChanged(p) => self.sales_tax.update(p),
        }
    }
    
    pub fn view(&self) -> Element<Message> {
        iced::widget::column![
            iced::widget::text("Gift Shop Sales").size(HEADER_SIZE),
            iced::widget::horizontal_rule(RULE_HEIGHT),
            text_input("Item Description", self.item_description.as_str()).on_input(Message::DescriptionChanged),
            self.price.view().map(|x| Message::PriceChanged(x)),
            pick_list(PaymentMethod::VARIANTS, self.payment_method.as_ref(), Message::PaymentMethodChanged).placeholder("Select Payment Method"),
            number_input(&self.quantity, 1..=u16::MAX, Message::QuantityChanged),
            self.sales_tax.view().map(|x| Message::SalesTaxChanged(x)),
        ].spacing(RULE_HEIGHT).into()
    }

    pub fn pre_tax_cost(&self) -> f32 {
        self.price.value() * self.quantity as f32
    }
    
    pub fn compute_tax(&self) -> f32 {
        self.pre_tax_cost() * self.sales_tax.value()
    }
    pub fn compute_total_cost(&self) -> f32 {
         self.pre_tax_cost() + self.compute_tax()
    }
}

impl AsTransactionRecord for GiftShopSale {
    fn as_transaction_record(&self) -> TransactionRecord {
        assert!(self.is_valid());
        TransactionRecord::new(
            TransactionKind::GiftShopSale,
            self.item_description.clone(),
            self.quantity,
            self.compute_total_cost(),
        )
    }

    fn is_valid(&self) -> bool {
        self.payment_method.is_some()
    }
}

impl GetPaymentMethod for GiftShopSale {
    fn get_payment_method(&self) -> Option<PaymentMethod> {
        self.payment_method.clone()
    }
}