use chrono::{DateTime, TimeZone, Timelike};

use sqlite::Value;
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


impl From<u32> for Hour {
    fn from(value: u32) -> Self {
        match value {
            10 => Hour::ElevenToTwelve,
            11 => Hour::TwelveToOne,
            12 => Hour::OneToTwo,
            13 => Hour::TwoToThree,
            _ => Hour::ThreeToFour,
        }
    }
}
impl<Tz: TimeZone> From<DateTime<Tz>> for Hour {
    fn from(dt: DateTime<Tz>) -> Self {
        Hour::from(dt.hour())
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

impl TryFrom<&Value> for Hour {
    type Error = sqlite::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let Value::Integer(integer) = value else {
            return Err(sqlite::Error {
                code: None,
                message: Some("Value is not an integer, so it can't be converted to an hour.".to_string()),
            })
        };
        let integer = *integer;
        if integer < 10 || integer > 14 {
            return Err(sqlite::Error {
                code: None,
                message: Some(format!("{integer} is not between 10 and 14 (inclusive).")),
            })
        }
        
        Ok(Hour::from(integer as u32))
    }
}