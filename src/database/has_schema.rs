pub trait HasSchema {
    fn schema(field_name: &str) -> String where Self: Sized;
}


const NOT_NULL: &str = "NOT NULL";
macro_rules! schema {
    ($on_type: ty, $sql_type: literal) => {
        impl HasSchema for $on_type {
            fn schema(field_name: &str) -> String {
                let field_type = $sql_type;
                format!("{field_name} {field_type} {NOT_NULL}")
             }   
        }
    };
}


schema!(bool,  "TINYINT");

schema!(i8,  "TINYINT");
schema!(i16, "SMALLINT");
schema!(i32, "INT");
schema!(i64, "BIGINT");

schema!(f32, "REAL");
schema!(f64, "DOUBLE");

schema!(String, "TEXT");
schema!(&str, "TEXT");

impl<T: HasSchema> HasSchema for Option<T> {
    fn schema(field_name: &str) -> String {
        T::schema(field_name).replace(NOT_NULL, "")
    }
}
