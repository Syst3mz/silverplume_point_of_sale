use chrono::{DateTime, TimeZone, Timelike};
use crate::database::has_schema::{HasSchema, NOT_NULL};
use crate::database::to_sql::ToSql;

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

impl HasSchema for Hour {
    fn schema(field_name: &str) -> String
    where
        Self: Sized
    {
        format!("{field_name} TINYINT {}", NOT_NULL)
    }
}
impl ToSql for Hour {
    fn to_sql(&self) -> String {
        match self {
            Hour::ElevenToTwelve => 10,
            Hour::TwelveToOne => 11,
            Hour::OneToTwo => 12,
            Hour::TwoToThree => 13,
            Hour::ThreeToFour => 14,
        }.to_string()
    }
}