use chrono::{DateTime, Local, TimeZone, Timelike};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TransactionKind {
    Admission,
    Membership,
    Donation,
    GiftShopSale,
}

#[derive(Debug, Clone)]
pub struct TransactionRecord {
    pub kind: TransactionKind,
    pub description: String,
    pub quantity: u16,
    pub total_cost: f32,
}

impl TransactionRecord {
    pub fn new(kind: TransactionKind, description: String, quantity: u16, amount: f32) -> Self {
        Self {
            kind,
            description,
            quantity,
            total_cost: amount,
        }
    }
    
}
