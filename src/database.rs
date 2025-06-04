use chrono::Datelike;
use std::{fs, thread};
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
    pub const TRANSACTIONS: &str = "Transactions";
    pub const ADMISSIONS: &str = "Admissions";
    pub const DONATIONS: &str = "Donations";
    pub const MEMBERSHIPS: &str = "Memberships";
    pub const GIFT_SHOP_SALES: &str = "Gift Shop Sales";
    pub const PREFIXES: [&str; 5] = [TRANSACTIONS, ADMISSIONS, DONATIONS, MEMBERSHIPS, GIFT_SHOP_SALES];
}
/// This is really really not a DB
pub struct Database {
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

fn age_of_file(filename: impl AsRef<Path>) -> Option<TimeDelta> {
    let now = Local::now();
    let file_created = DateTime::<Local>::from(fs::metadata(filename).ok()?.created().ok()?);
    Some(now - file_created)
}

fn age_of_oldest_file() -> Option<TimeDelta> {
    file_prefixes::PREFIXES.iter()
        .filter_map(|x| age_of_file(append_str(x, DOT_CSV)))
        .max_by(|a, b| a.num_days().cmp(&b.num_days()))
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

fn rename_files() -> std::io::Result<()> {
    let last_month = last_month();
    for file_prefix in file_prefixes::PREFIXES.iter() {
        let new_file_name = format!("{}_{}", file_prefix, last_month);
        match fs::rename(append_str(file_prefix, DOT_CSV), append_str(new_file_name, DOT_CSV)) {
            Ok(_) => { },
            Err(e) => return Err(e),
        }
    }
    
    Ok(())
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
    
    fn write_all_files(&self) {
        write_vec(&self.admissions, append_str(file_prefixes::ADMISSIONS, DOT_CSV));
        write_vec(&self.donations, append_str(file_prefixes::DONATIONS, DOT_CSV));
        write_vec(&self.memberships, append_str(file_prefixes::MEMBERSHIPS, DOT_CSV));
        write_vec(&self.gift_shop_sales, append_str(file_prefixes::GIFT_SHOP_SALES, DOT_CSV));
        write_vec(&self.transactions, append_str(file_prefixes::TRANSACTIONS, DOT_CSV));
    }
}
impl Default for Database {
    fn default() -> Self {
        let mut database = Self {
            admissions: vec![],
            donations: vec![],
            memberships: vec![],
            gift_shop_sales: vec![],
            transactions: vec![],
        };
        
        if let Some(age) = age_of_oldest_file() {
            #[cfg(debug_assertions)]
            let max_age = TimeDelta::seconds(20);
            #[cfg(not(debug_assertions))]
            let max_age = TimeDelta::days(30);
            
            
            if dbg!(age) < dbg!(max_age) {
                database.admissions = read_vec(append_str(file_prefixes::ADMISSIONS, DOT_CSV));
                database.donations = read_vec(append_str(file_prefixes::DONATIONS, DOT_CSV));
                database.memberships = read_vec(append_str(file_prefixes::MEMBERSHIPS, DOT_CSV));
                database.gift_shop_sales = read_vec(append_str(file_prefixes::GIFT_SHOP_SALES, DOT_CSV));
                database.transactions = read_vec(append_str(file_prefixes::TRANSACTIONS, DOT_CSV));
            } else {
                println!("Making a new database since the last one was too old!");
                rename_files().expect("Unable to rename files");
                
                // this is to let the OS actually commit the change...IDK if it's enough.
                thread::sleep(std::time::Duration::from_millis(66));
                database.write_all_files();
            }
        } else {
            
            println!("Making a brand new database since we have never been run here before");
            database.write_all_files();
        }
        
        database
    }
}