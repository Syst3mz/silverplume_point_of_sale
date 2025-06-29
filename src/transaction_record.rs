use chrono::{DateTime, Local, TimeZone, Timelike};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TransactionKind {
    Admission,
    Membership,
    Donation,
    GiftShopSale,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hour {
    ElevenToTwelve,
    TwelveToOne,
    OneToTwo,
    TwoToThree,
    ThreeToFour,
}

impl<Tz: TimeZone> From<DateTime<Tz>> for Hour {
    fn from(dt: DateTime<Tz>) -> Self {
        match dt.hour() { 
            10 => Hour::ElevenToTwelve,
            11 => Hour::TwelveToOne,
            12 => Hour::OneToTwo,
            13 => Hour::TwoToThree,
            _ => Hour::ThreeToFour,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransactionRecord {
    pub kind: TransactionKind,
    pub description: String,
    pub quantity: u16,
    pub total_cost: f32,
    pub hour: Hour
}

impl TransactionRecord {
    pub fn new(kind: TransactionKind, description: String, quantity: u16, amount: f32) -> Self {
        let now = Local::now();
        let hour = Hour::from(now);
        Self {
            kind,
            description,
            quantity,
            total_cost: amount,
            hour
        }
    }
    
}
