use sqlite::Connection;
mod has_schema;
mod object_mapper;
mod to_sql;
mod from_sql;

pub struct Database {
    database: Connection
}


impl Database {
    fn create_db(&self) {
        
    }
}


const DB_LOCATION: &'static str = "pos.db";
impl Default for Database {
    fn default() -> Self {
        let conn = Connection::open(DB_LOCATION).expect("Can't open database");
        Self {
            database: conn
        }
    }
}