use sqlite::Row;
use crate::database::database_object::CanBuildObjectMapper;
use crate::database::from_sql::FromSql;
use crate::database::object_mapper::ObjectMapper;
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::date_time_wrapper::WrapInDateTime;
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
impl CanBuildObjectMapper for GiftShopSale {
    const TABLE_NAME: &'static str = "gift_shop_sales";

    fn build_object_mapper(&self) -> ObjectMapper {
        ObjectMapper::new(Self::TABLE_NAME)
            .add_field("item_description", self.item_description.clone())
            .add_field("price", self.price)
            .add_field("payment_method", self.payment_method.clone())
            .add_field("quantity", self.quantity as i32)
            .add_field("sales_tax", self.sales_tax)
    }
}

impl FromSql for GiftShopSale {
    fn from_sql(row: Row) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let price: f64 = row.try_read("price")?;
        let quantity: i64 = row.try_read("quantity")?;
        let sales_tax: f64 = row.try_read("sales_tax")?;
        Ok(Self {
            item_description: row.try_read::<&str, _>("item_description")?.to_string(),
            price: price as f32,
            payment_method: row.try_read("payment_method")?,
            quantity: quantity as u16,
            sales_tax: sales_tax as f32,
        })
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