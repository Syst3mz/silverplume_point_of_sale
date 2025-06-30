use chrono::{DateTime, Local};
use crate::database::database_object::DatabaseObject;
use crate::database::has_schema::HasSchema;
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

impl<T: DatabaseObject> DatabaseObject for DateTimeWrapper<T> {
    fn build_object_mapper(&self) -> ObjectMapper {
        self.element.build_object_mapper()
            .add_field("date", self.date_time)
            .add_field("hour", self.hour)
    }
}

pub trait WrapInDateTime {
    fn wrapped_in_date_time(self) -> DateTimeWrapper<Self> where Self: Sized {
        DateTimeWrapper::new(self)
    }
}

