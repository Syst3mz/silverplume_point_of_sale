use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransactionKind {
    Admission,
    Membership,
    Donation,
    GiftShopSale,
}

pub mod rfc3339 {
    use chrono::DateTime;
    use chrono::offset::Local;
    use serde::{self, Deserialize, Deserializer, Serializer};

    // Serialize any DateTime<Tz> to RFC3339 string
    pub fn serialize<Tz, S>(date: &DateTime<Tz>, serializer: S) -> Result<S::Ok, S::Error>
    where
        Tz: chrono::TimeZone,
        S: Serializer,
        DateTime<Tz>: ToString,
    {
        serializer.serialize_str(&date.to_rfc3339())
    }

    // Deserialize as DateTime<Local>
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Local))
            .map_err(serde::de::Error::custom)
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    #[serde(with = "crate::transaction_record::rfc3339")]
    pub time: DateTime<Local>,
    #[serde(rename = "type")]
    pub kind: TransactionKind,
    pub description: String,
    pub quantity: u16,
    pub amount: f32,
}

impl TransactionRecord {
    pub fn new(kind: TransactionKind, description: String, quantity: u16, amount: f32) -> Self {
        Self {
            time: Local::now(),
            kind,
            description,
            quantity,
            amount,
        }
    }
}
