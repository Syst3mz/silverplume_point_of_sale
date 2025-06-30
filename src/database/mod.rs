use itertools::Itertools;
use sqlite::Connection;
use crate::database::database_object::DatabaseObject;
use crate::database::object_mapper::ObjectMapper;
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
        
        println!("Creating database objects:");
        connection.execute(defaults.iter().join("\n")).expect("Unable to create database.")
    }
    
    pub fn insert<T: DatabaseObject>(&self, object: DateTimeWrapper<T>) -> anyhow::Result<()> {
        println!("executing:");
        Ok(self.database.execute(dbg!(object.build_object_mapper().insert()))?)
    }
}



