use std::any;
use std::any::Any;
use chrono::{Duration, Local};
use itertools::Itertools;
use log::{error, info};
use minijinja::Environment;
use serde::Serialize;
use sqlite::{Connection, Value};
use crate::database::database_object::CanBuildObjectMapper;
use crate::database::from_sql::FromSql;
use crate::database::to_sql::ToSql;
use crate::model::admission::Admission;
use crate::model::date_time_wrapper::DateTimeWrapper;
use crate::model::donation::Donation;
use crate::model::gift_shop_sale::GiftShopSale;
use crate::model::membership::Membership;
use crate::model::transaction_record::TransactionRecord;
use crate::view::summary_dicts::SummaryDicts;

pub mod has_schema;
pub mod object_mapper;
pub mod to_sql;
pub mod from_sql;
pub mod database_object;

pub struct Database {
    database: Connection,
    daily_admissions: Vec<Admission>,
    daily_memberships: Vec<Membership>,
    daily_donations: Vec<Donation>,
    daily_gift_shop_sales: Vec<GiftShopSale>,
    daily_transactions: Vec<TransactionRecord>
}


impl Database {
    const FILEPATH: &'static str = "pos.db";

    pub fn new() -> Self {
        let conn = Connection::open(Self::FILEPATH).expect("Can't open database");
        Self::create_schemas(&conn);
        let mut start = Self {
            database: conn,
            daily_admissions: vec![],
            daily_memberships: vec![],
            daily_donations: vec![],
            daily_gift_shop_sales: vec![],
            daily_transactions: vec![],
        };

        start.read_entire_day();
        start
    }

    fn read_entire_day(&mut self) {
        let _ = self.select_since(<Admission as CanBuildObjectMapper>::TABLE_NAME, Duration::days(1))
            .map(|x| self.daily_admissions = x).map_err(|x| {error!("err reading admissions: {}", x); x});
        let _ = self.select_since(<Membership as CanBuildObjectMapper>::TABLE_NAME, Duration::days(1))
            .map(|x| self.daily_memberships = x).map_err(|x| {error!("err reading memberships: {}", x); x});
        let _ = self.select_since(<Donation as CanBuildObjectMapper>::TABLE_NAME, Duration::days(1))
            .map(|x| self.daily_donations = x).map_err(|x| {error!("err reading donations{}", x); x});
        let _ = self.select_since(<GiftShopSale as CanBuildObjectMapper>::TABLE_NAME, Duration::days(1))
            .map(|x| self.daily_gift_shop_sales = x).map_err(|x| {error!("err reading gift shop sales: {}", x); x});
        let _ = self.select_since(<TransactionRecord as CanBuildObjectMapper>::TABLE_NAME, Duration::days(1))
            .map(|x| self.daily_transactions = x).map_err(|x| {error!("err reading transactions: {}", x); x});
    }
    fn create_schemas(connection: &Connection) {
        let defaults = [
            DateTimeWrapper::new(Admission::default()).build_object_mapper().schema(),
            DateTimeWrapper::new(Donation::default()).build_object_mapper().schema(),
            DateTimeWrapper::new(GiftShopSale::default()).build_object_mapper().schema(),
            DateTimeWrapper::new(Membership::default()).build_object_mapper().schema(),
            DateTimeWrapper::new(TransactionRecord::default()).build_object_mapper().schema(),
        ];
        
        info!("Creating schemas");
        connection.execute(defaults.iter().join("\n")).expect("Unable to create database.")
    }
    
    pub fn insert<T: CanBuildObjectMapper+Any>(&mut self, object: DateTimeWrapper<T>) -> anyhow::Result<()> {
        info!("Logging a {}", any::type_name::<T>());
        let res = Ok(self.database.execute(object.build_object_mapper().insert())?);

        //todo: This is horribly inefficient, I should just be inserting where it makes sense.
        self.read_entire_day();
        res
    }

    pub fn select_since<T: FromSql>(&self, table_name: impl AsRef<str>, since: Duration) -> Result<Vec<T>, anyhow::Error> {
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

    pub fn daily_admissions(&self) -> &Vec<Admission> {
        &self.daily_admissions
    }
    pub fn daily_memberships(&self) -> &Vec<Membership> {
        &self.daily_memberships
    }
    pub fn daily_donations(&self) -> &Vec<Donation> {
        &self.daily_donations
    }
    pub fn daily_gift_shop_sales(&self) -> &Vec<GiftShopSale> {
        &self.daily_gift_shop_sales
    }
    pub fn daily_transactions(&self) -> &Vec<TransactionRecord> {
        &self.daily_transactions
    }
    pub fn render_to_html(&self) -> String {
        const TEMPLATE_STR: &'static str = include_str!("../summary.html");

        let mut templates = Environment::new();
       templates.add_template("summary", TEMPLATE_STR).unwrap();
        #[derive(Serialize)]
        struct Context {
            frequency: String,
            fields: SummaryDicts,
        }


        let context = Context {
            frequency: "Daily".to_string(),
            fields: SummaryDicts::new(self)
        };


        templates.get_template("summary").unwrap().render(&context).unwrap()
    }
}