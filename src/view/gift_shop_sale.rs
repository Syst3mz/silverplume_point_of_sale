use anyhow::anyhow;
use iced::Element;
use iced::widget::{pick_list, row, text, text_input};
use iced_aw::number_input;
use strum::VariantArray;
use crate::decimal_input::DecimalInput;
use crate::{HEADER_SIZE, RULE_HEIGHT, TEXT_SIZE};
use crate::model::payment_method::PaymentMethod;
use crate::to_model::ToModel;

#[derive(PartialEq, Debug, Clone)]
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
            sales_tax: DecimalInput::new("Sales Tax(%)", DEFAULT_SALES_TAX),
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

    fn compute_total_cost(&self) -> f32 {
        let tax_factor = (self.sales_tax.value() / 100.0) + 1.0;
        self.price.value() * (self.quantity as f32) * tax_factor
    }
    
    pub fn view(&self) -> Element<Message> {
        iced::widget::column![
            iced::widget::text("Gift Shop Sales").size(HEADER_SIZE),
            iced::widget::horizontal_rule(RULE_HEIGHT),
            text_input("Item Description", self.item_description.as_str()).on_input(Message::DescriptionChanged),
            self.price.view().map(|x| Message::PriceChanged(x)),
            pick_list(PaymentMethod::VARIANTS, self.payment_method.as_ref(), Message::PaymentMethodChanged).placeholder("Select Payment Method"),
            row![text("Quantity: ").size(TEXT_SIZE), number_input(&self.quantity, 1..=u16::MAX, Message::QuantityChanged)].spacing(RULE_HEIGHT),
            self.sales_tax.view().map(|x| Message::SalesTaxChanged(x)),
            text(format!("Total due: ${:.2}", self.compute_total_cost())).size(TEXT_SIZE),
        ].spacing(RULE_HEIGHT).into()
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.payment_method.is_some() && self.quantity > 0 && self.price.value() >= 0.0 && self.sales_tax.value() >= 0.0
    }
}

impl ToModel for GiftShopSale {
    type ModelType = crate::model::gift_shop_sale::GiftShopSale;

    fn to_model(&self) -> anyhow::Result<Self::ModelType> {
        Ok(
            Self::ModelType::new(
                self.item_description.clone(), 
                self.price.value(), 
                self.payment_method.ok_or(anyhow!("Missing payment method"))?,
                self.quantity, 
                self.sales_tax.value()
            )
        )
    }
}