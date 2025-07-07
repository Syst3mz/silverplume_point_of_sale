use std::error::Error;
use sqlite::{Row, Value};

pub trait FromSql {
    fn from_sql(row: Row) -> anyhow::Result<Self> where Self: Sized;
}

pub fn from_option<'a, T>(value: &'a Value) -> anyhow::Result<Option<T>>
where
    T: TryFrom<&'a Value>,
    <T as TryFrom<&'a Value>>::Error: Error + Send + Sync + 'static, {
    if let Value::Null = value { 
        return Ok(None);
    }
    
    Ok(Some(T::try_from(value)?))
}