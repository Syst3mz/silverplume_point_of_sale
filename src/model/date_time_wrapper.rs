use chrono::{DateTime, Local};
use sqlite::{Row, Value};
use crate::database::database_object::CanBuildObjectMapper;
use crate::database::from_sql::FromSql;
use crate::database::object_mapper::ObjectMapper;
use crate::model::hour::Hour;

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

impl<T: CanBuildObjectMapper> CanBuildObjectMapper for DateTimeWrapper<T> {
    const TABLE_NAME: &'static str = "INVALID TABLE NAME, THIS IS JUST A WRAPPER.";

    fn build_object_mapper(&self) -> ObjectMapper {
        self.element.build_object_mapper()
            .add_field("date_time", self.date_time)
            .add_field("hour", self.hour)
    }
}

pub trait WrapInDateTime {
    fn wrapped_in_date_time(self) -> DateTimeWrapper<Self> where Self: Sized {
        DateTimeWrapper::new(self)
    }
}
fn read_date_time(v: Value) -> anyhow::Result<DateTime<Local>> {
    let Value::String(str) = v else {
        return Err(anyhow::Error::msg("Expected date_time to be stored as a string."));
    };
    
    Ok(DateTime::parse_from_rfc3339(str.as_str())?.into())
}
impl<T: FromSql> FromSql for DateTimeWrapper<T> {
    fn from_sql(mut row: Row) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let date_time = read_date_time(row.take("date_time"))?;
        let hour = row.try_read("hour")?;
        Ok(Self {
            element: T::from_sql(row)?,
            date_time,
            hour,
        })
    }
}