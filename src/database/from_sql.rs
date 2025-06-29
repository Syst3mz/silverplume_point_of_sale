pub type Unknown = (); 
pub trait FromSql {
    fn from_sql(_: Unknown) -> anyhow::Result<Self> where Self: Sized;
}