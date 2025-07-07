use std::str::FromStr;
use sqlite::Value;
use strum::{Display, EnumString, VariantArray};
use crate::as_description::AsDescription;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, VariantArray, Display, EnumString)]
pub enum Kind {
    #[default]
    Adult,
    Senior,
    #[strum(serialize = "Child (6-12)")]
    ChildUnderThirteen,
    #[strum(serialize = "Child (Under 6)")]
    ChildUnderSix,
    #[strum(serialize = "PFSP Member")]
    PfspMember,
    #[strum(serialize = "Silver Plume Resident")]
    Resident
}

impl Kind {
    pub fn is_free(&self) -> bool {
        self.cost() == 0.0
    }

    pub fn cost(&self) -> f32 {
        match self {
            Kind::Adult => 8.0,
            Kind::Senior => 5.0,
            Kind::ChildUnderThirteen => 3.0,
            Kind::ChildUnderSix | Kind::PfspMember | Kind::Resident => 0.0,
        }
    }
}

impl TryFrom<&Value> for Kind {
    type Error = sqlite::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let Value::String(str) = value else { 
            return Err(sqlite::Error {
                code: None,
                message: Some("value is not a string, and therefore cannot be converted to a Kind.".to_string()),
            })
        };
        
        Ok(Kind::from_str(str).map_err(|x| sqlite::Error {
            code: None,
            message: Some(x.to_string()),
        })?)
    }
}

impl AsDescription for Kind {
    fn as_description(&self) -> &str {
        match self {
            Kind::Adult => "Adult - $8.00",
            Kind::Senior => "Adult - $5.00",
            Kind::ChildUnderThirteen => "Child (6-12) - $3.00",
            Kind::ChildUnderSix => "Child (Under 6) - Free",
            Kind::PfspMember => "PFSP Member - Free",
            Kind::Resident => "Resident - Free",
        }
    }
}