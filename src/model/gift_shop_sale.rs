use crate::database::database_object::DatabaseObject;
use crate::database::object_mapper::ObjectMapper;
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::date_time_wrapper::WrapInDateTime;
use crate::model::donation::Donation;
use crate::model::payment_method::PaymentMethod;
use crate::model::transaction_record::{TransactionKind, TransactionRecord};

#[derive(PartialEq, Debug, Clone)]
pub struct GiftShopSale {
    item_description: String,
    price: f32,
    pub payment_method: PaymentMethod,
    quantity: u16,
    sales_tax: f32
}
impl GiftShopSale {
    pub fn new(item_description: String, price: f32, payment_method: PaymentMethod, quantity: u16, sales_tax: f32) -> Self {
        Self {
            item_description,
            price,
            payment_method,
            quantity,
            sales_tax,
        }
    }
    pub fn pre_tax_cost(&self) -> f32 {
        self.price * self.quantity as f32
    }
    
    pub fn compute_tax(&self) -> f32 {
        self.pre_tax_cost() * (self.sales_tax / 100.0)
    }
    pub fn compute_total_cost(&self) -> f32 {
         self.pre_tax_cost() + self.compute_tax()
    }
}
impl AsTransactionRecord for GiftShopSale {
    fn as_transaction_record(&self) -> TransactionRecord {
        TransactionRecord::new(
            TransactionKind::GiftShopSale,
            self.item_description.clone(),
            self.quantity,
            self.compute_total_cost(),
        )
    }
}
impl WrapInDateTime for GiftShopSale {}
impl DatabaseObject for GiftShopSale {
    fn build_object_mapper(&self) -> ObjectMapper {
        ObjectMapper::new("gift_shop_sales")
            .add_field("item_description", self.item_description.clone())
            .add_field("price", self.price)
            .add_field("payment_method", self.payment_method.clone())
            .add_field("quantity", self.quantity as i32)
            .add_field("sales_tax", self.sales_tax)
    }
}
impl Default for GiftShopSale {
    fn default() -> Self {
        Self {
            item_description: Default::default(),
            price: 0.0,
            payment_method: Default::default(),
            quantity: 0,
            sales_tax: 0.0,
        }
    }
}