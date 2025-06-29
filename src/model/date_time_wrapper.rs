use chrono::{DateTime, Local, TimeZone, Timelike};

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

pub struct DateTimeWrapper<T> {
    element: T,
    date_time: DateTime<Local>,
    hour: Hour
}

impl<T> DateTimeWrapper<T> {
    pub fn new(element: T) -> DateTimeWrapper<T> {
        let now = Local::now();
        let hour = Hour::from(now);
        Self {
            element,
            date_time: now,
            hour,
        }
    }
}

pub trait WrapInDateTime {
    fn wrapped_in_date_time(self) -> DateTimeWrapper<Self> where Self: Sized {
        DateTimeWrapper::new(self)
    }
}