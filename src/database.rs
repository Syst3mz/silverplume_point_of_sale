use chrono::{Datelike, Utc};
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use chrono::{DateTime, Duration, Local, TimeDelta};
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::admission::Admission;
use crate::as_transaction_record::AsTransactionRecord;
use crate::donation::Donation;
use crate::gift_shop_sale::GiftShopSale;
use crate::membership::Membership;
use crate::transaction_record::{TransactionKind, TransactionRecord};

mod file_prefixes {
    pub const TRANSACTIONS: &str = "data/Transactions";
    pub const ADMISSIONS: &str = "data/Admissions";
    pub const DONATIONS: &str = "data/Donations";
    pub const MEMBERSHIPS: &str = "data/Memberships";
    pub const GIFT_SHOP_SALES: &str = "data/Gift Shop Sales";
    pub const DATA_FILE_PREFIXES: [&str; 5] = [TRANSACTIONS, ADMISSIONS, DONATIONS, MEMBERSHIPS, GIFT_SHOP_SALES];
}

const DATABASE_LOCK_FILE: &str = "database.lock";

/// This is really really not a DB
pub struct Database {
    created: DateTime<Utc>,
    pub admissions: Vec<Admission>,
    pub donations: Vec<Donation>,
    pub memberships: Vec<Membership>,
    pub gift_shop_sales: Vec<GiftShopSale>,
    pub transactions: Vec<TransactionRecord>
}

const DOT_CSV: &str = ".csv";
fn append_str(str: impl AsRef<str>, with: impl AsRef<str>) -> String {
    format!("{}{}", str.as_ref(), with.as_ref())
}

fn read_vec<T: DeserializeOwned>(filename: impl AsRef<Path>) -> Vec<T> {
    let mut rdr = csv::Reader::from_path(filename).expect("Unable to open file");
    let mut result = Vec::new();
    for record in rdr.deserialize() {
        result.push(record.expect("Unable to read record"));
    }
    result
}

fn write_vec<T: Serialize>(to_write: &[T], file_prefix: impl AsRef<Path>) {
    let mut wtr = csv::Writer::from_path(file_prefix).expect("could not open csv file");
    for record in to_write {
        let _ = wtr.serialize(record);
    }
    let _ = wtr.flush();
}

fn last_month() -> String {
    let now = Local::now().date_naive();
    let last_month = if now.month() == 1 {
        // Wrap around to December of previous year
        chrono::NaiveDate::from_ymd_opt(now.year() - 1, 12, 1).unwrap()
    } else {
        chrono::NaiveDate::from_ymd_opt(now.year(), now.month() - 1, 1).unwrap()
    };
    
    format!("{}_{}", last_month.format("%B"), now.year())
}

impl Database {
    pub fn todays_transactions(&self) -> impl Iterator<Item=&TransactionRecord>{
        let now = Local::now();
        self.transactions.iter()
            .filter(move |x| (now - x.time) <= Duration::days(1))
    }
    pub fn todays_transactions_of_kind(&self, kind: TransactionKind) -> impl Iterator<Item=&TransactionRecord>{
        self.todays_transactions().filter(move |x| x.kind == kind)
    }

    fn append_to_vec_and_write<T: Serialize>(vec: &mut Vec<T>, item: T, file_prefix: impl AsRef<str>) {
        vec.push(item);
        write_vec(vec, append_str(file_prefix, DOT_CSV));
    }

    pub fn add_admission(&mut self, admission: Admission) {
        Self::append_to_vec_and_write(&mut self.admissions, admission, file_prefixes::ADMISSIONS);
        Self::append_to_vec_and_write(&mut self.transactions, admission.as_transaction_record(), file_prefixes::TRANSACTIONS);
    } 
    pub fn add_donation(&mut self, donation: Donation) {
        Self::append_to_vec_and_write(&mut self.donations, donation.clone(), file_prefixes::DONATIONS);
        Self::append_to_vec_and_write(&mut self.transactions, donation.as_transaction_record(), file_prefixes::TRANSACTIONS);
    } 
    pub fn add_membership(&mut self, membership: Membership) {
        Self::append_to_vec_and_write(&mut self.memberships, membership, file_prefixes::MEMBERSHIPS);
        Self::append_to_vec_and_write(&mut self.transactions, membership.as_transaction_record(), file_prefixes::TRANSACTIONS);
    } 
    pub fn add_gift_shop_sale(&mut self, gift_shop_sale: GiftShopSale) {
        Self::append_to_vec_and_write(&mut self.gift_shop_sales, gift_shop_sale.clone(), file_prefixes::MEMBERSHIPS);
        Self::append_to_vec_and_write(&mut self.transactions, gift_shop_sale.as_transaction_record(), file_prefixes::TRANSACTIONS);
    }
    
    fn write_created_time_to_disk(&self) -> std::io::Result<()> {
        fs::write(DATABASE_LOCK_FILE, self.created.timestamp_millis().to_string())
    }
    
    fn read_created_time_from_disk() -> std::io::Result<DateTime<Utc>> {
        let timestamp = fs::read_to_string(DATABASE_LOCK_FILE)?;
        let timestamp = timestamp.parse::<i64>().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        DateTime::from_timestamp_millis(timestamp)
            .ok_or(std::io::Error::new(ErrorKind::InvalidData, "invalid timestamp"))

        
    }
    
    fn write_all_files(&self) {
        self.write_created_time_to_disk().expect("Unable to write created time to disk.");
        write_vec(&self.admissions, append_str(file_prefixes::ADMISSIONS, DOT_CSV));
        write_vec(&self.donations, append_str(file_prefixes::DONATIONS, DOT_CSV));
        write_vec(&self.memberships, append_str(file_prefixes::MEMBERSHIPS, DOT_CSV));
        write_vec(&self.gift_shop_sales, append_str(file_prefixes::GIFT_SHOP_SALES, DOT_CSV));
        write_vec(&self.transactions, append_str(file_prefixes::TRANSACTIONS, DOT_CSV));
    }

    fn lock_file_exists() -> bool {
        fs::exists(DATABASE_LOCK_FILE).unwrap_or_else(|_| false)
    }
    
    fn make_new_data_files() -> bool {
        let Ok(creation_time) = Self::read_created_time_from_disk() else {return true};

        #[cfg(debug_assertions)]
        let max_age = TimeDelta::seconds(10);
        #[cfg(not(debug_assertions))]
        let max_age = TimeDelta::days(30);
        
        Utc::now() - creation_time > max_age
    }

    /*pub fn admissions_with_payment_method(&self, method: PaymentMethod) -> impl Iterator<Item=&PaymentMethod> {
        self.admissions.iter().filter(|x| x.payment_method.is_some()).filter(|x| x == method)
    }
    pub fn donations_with_payment_method(&self, method: PaymentMethod) -> impl Iterator<Item=&PaymentMethod> {
        self.donations.iter().filter(|x| x.payment_method.is_some()).filter(|x| *x == method)
    }
    pub fn memberships_with_payment_method(&self, method: PaymentMethod) -> impl Iterator<Item=&PaymentMethod> {
        self.memberships.iter().filter(|x| x.payment_method.is_some()).filter(|x| *x == method)
    }
    pub fn gift_shop_sales_with_payment_method(&self, method: PaymentMethod) -> impl Iterator<Item=&PaymentMethod> {
        self.gift_shop_sales.iter().filter(|x| x.payment_method.is_some()).filter(|x| *x == method)
    }*/
}

fn rename_files() {
    let last_month = last_month();
    for file_prefix in file_prefixes::DATA_FILE_PREFIXES.iter() {
        let from = append_str(file_prefix, DOT_CSV);
        let to = append_str(format!("{}_{}", file_prefix, last_month), DOT_CSV);
        fs::rename(from, to).expect("Unable to rename data file");
    }
}

fn create_data_dir() {
    match fs::create_dir("data") {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {}
            _ => panic!("{}", e),
        }
    }
}
impl Default for Database {
    fn default() -> Self {
        let mut database = Self {
            created: Utc::now(),
            admissions: vec![],
            donations: vec![],
            memberships: vec![],
            gift_shop_sales: vec![],
            transactions: vec![],
        };
        
        if !Self::lock_file_exists() {
            println!("Making a brand new database since we have never been run here before");
            create_data_dir();
            database.write_all_files();
            return database;
        }
        
        if Self::make_new_data_files() {
            println!("Making new data files, its the start of a new month!");
            rename_files();
            database.created = Utc::now();
            database.write_all_files();
            return database
        }
        
        println!("Reading data from data directory!");
        // safe to unwrap b/c otherwise the program is fucked.
        database.created = Self::read_created_time_from_disk().unwrap();
        database.admissions = read_vec(append_str(file_prefixes::ADMISSIONS, DOT_CSV));
        database.donations = read_vec(append_str(file_prefixes::DONATIONS, DOT_CSV));
        database.memberships = read_vec(append_str(file_prefixes::MEMBERSHIPS, DOT_CSV));
        database.gift_shop_sales = read_vec(append_str(file_prefixes::GIFT_SHOP_SALES, DOT_CSV));
        database.transactions = read_vec(append_str(file_prefixes::TRANSACTIONS, DOT_CSV));
        database
    }
}