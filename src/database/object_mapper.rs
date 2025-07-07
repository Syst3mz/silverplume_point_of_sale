use indexmap::IndexMap;
use itertools::Itertools;
use crate::database::has_schema::HasSchema;
use crate::database::to_sql::ToSql;

pub trait Mappable {
    fn schema(&self, field_name: &str) -> String;
    fn to_sql(&self) -> String;
}
impl<T: HasSchema+ToSql> Mappable for T {
    fn schema(&self, field_name: &str) -> String {
        <Self as HasSchema>::schema(field_name)
    }

    fn to_sql(&self) -> String {
        self.to_sql()
    }
}

pub struct ObjectMapper {
    table_name: String,
    fields: IndexMap<String, Box<dyn Mappable>>
}

impl ObjectMapper {
    pub fn new(table_name: impl AsRef<str>) -> ObjectMapper {
        Self {
            table_name: table_name.as_ref().to_string(),
            fields: Default::default(),
        }
    }
    
    pub fn add_field(mut self, name: impl AsRef<str>, field: impl Mappable + 'static) -> ObjectMapper {
        self.fields.insert(name.as_ref().to_string(), Box::new(field));
        self
    }
    
    pub fn schema(&self) -> String {
        format!("CREATE TABLE IF NOT EXISTS {} (\n\t{}\n);", self.table_name, 
                self.fields.iter().map(|(name, field)| field.schema(name)).join(",\n\t")
        )
    }
    
    pub fn insert(&self) -> String {
        format!("INSERT INTO {} ({})\nVALUES ({});", 
                self.table_name, 
                self.fields.keys().join(", "), 
                self.fields.values().map(|x| x.to_sql()).join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn demo() -> ObjectMapper {
        ObjectMapper::new("test")
            .add_field("name", "Sally")
            .add_field("age", 24)
            .add_field("birthday", 1351683413616383413_i64)
    }
    #[test]
    fn schema() {
        assert_eq!(demo().schema(), "CREATE TABLE IF NOT EXISTS test (
	name TEXT NOT NULL,
	age INT NOT NULL,
	birthday BIGINT NOT NULL
);")
    }
    
    #[test]
    fn insert() {
        assert_eq!(demo().insert(), "INSERT INTO test (name, age, birthday)\nVALUES ('Sally', 24, 1351683413616383413);");
    }
}