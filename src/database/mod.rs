use chrono::{Duration, Local};
use itertools::Itertools;
use sqlite::{Connection, Value};
use crate::database::database_object::CanBuildObjectMapper;
use crate::database::from_sql::FromSql;
use crate::database::to_sql::ToSql;
use crate::model::date_time_wrapper::DateTimeWrapper;

pub mod has_schema;
pub mod object_mapper;
pub mod to_sql;
pub mod from_sql;
pub mod database_object;

pub struct Database {
    database: Connection
}


impl Database {
    const FILEPATH: &'static str = "pos.db";
    pub fn new() -> Self {
        let conn = Connection::open(Self::FILEPATH).expect("Can't open database");
        Self::create_schemas(&conn);
        Self {
            database: conn
        }
    }
    fn create_schemas(connection: &Connection) {
        use crate::model::*;
        let defaults = [
            DateTimeWrapper::new(admission::Admission::default()).build_object_mapper().schema(),
            DateTimeWrapper::new(donation::Donation::default()).build_object_mapper().schema(),
            DateTimeWrapper::new(gift_shop_sale::GiftShopSale::default()).build_object_mapper().schema(),
            DateTimeWrapper::new(membership::Membership::default()).build_object_mapper().schema(),
            DateTimeWrapper::new(transaction_record::TransactionRecord::default()).build_object_mapper().schema(),
        ];
        
        println!("Creating database objects");
        connection.execute(defaults.iter().join("\n")).expect("Unable to create database.")
    }
    
    pub fn insert<T: CanBuildObjectMapper>(&self, object: DateTimeWrapper<T>) -> anyhow::Result<()> {
        Ok(self.database.execute(object.build_object_mapper().insert())?)
    }
    
    pub fn select<T: FromSql>(&self, table_name: impl AsRef<str>, since: Duration) -> Result<Vec<T>, anyhow::Error> {
        let duration = Local::now() - since;
        let table_name = table_name.as_ref();
        let response = self.database.prepare(format!("SELECT * FROM {table_name} WHERE date_time >:max_age"))?;
        let binding: &[(&'static str, Value)] = &[
            (":max_age", duration.to_sql().into())
        ];
        
        Result::from_iter(response
            .into_iter()
            .bind(binding)?
            .filter_map(|x| x.ok())
            .map(|x| T::from_sql(x))
        )
    }
}



