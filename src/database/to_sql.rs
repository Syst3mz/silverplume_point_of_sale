use chrono::{DateTime, Local};

pub trait ToSql {
    fn to_sql(&self) -> String;
}

macro_rules! impl_to_sql_via_to_string {
    ($ty:ty) => {
        impl ToSql for $ty {
            fn to_sql(&self) -> String {
                self.to_string()
            } 
        }
    };
}

impl_to_sql_via_to_string!(i8);
impl_to_sql_via_to_string!(i16);
impl_to_sql_via_to_string!(i32);
impl_to_sql_via_to_string!(i64);

impl_to_sql_via_to_string!(f32);
impl_to_sql_via_to_string!(f64);

impl ToSql for bool {
    fn to_sql(&self) -> String {
        if *self { 
            "TRUE".to_string()
        } else {
            "FALSE".to_string()
        }
    }
}

impl ToSql for &str {
    fn to_sql(&self) -> String {
        format!("'{}'", *self)
    }
}

impl ToSql for String {
    fn to_sql(&self) -> String {
        self.as_str().to_sql()
    }
}

impl<T: ToSql> ToSql for Option<T> {
    fn to_sql(&self) -> String {
        let Some(value) = &self else {return "NULL".to_string(); };
        value.to_sql()
    }
}
impl ToSql for DateTime<Local> {
    fn to_sql(&self) -> String {
        format!("'{}'", self.to_rfc3339())
    }
}