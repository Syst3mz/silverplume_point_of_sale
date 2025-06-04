use serde::{Deserialize, Serialize};
use strum::{Display, VariantArray};
use crate::as_description::AsDescription;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, VariantArray, Display, Default)]
pub enum Type_ {
    #[default]
    Adult,
    Senior,
    #[strum(serialize = "Child (6-12)")]
    ChildUnderThirteen,
    #[strum(serialize = "Child (Under 6)")]
    ChildUnderSix,
    #[strum(serialize = "PFSP Member")]
    PfspMember
}

impl Type_ {
    pub fn is_free(&self) -> bool {
        self.cost() == 0.0
    }

    pub fn cost(&self) -> f32 {
        match self {
            Type_::Adult => 8.0,
            Type_::Senior => 5.0,
            Type_::ChildUnderThirteen => 3.0,
            Type_::ChildUnderSix | Type_::PfspMember => 0.0
        }
    }
}

impl AsDescription for Type_ {
    fn as_description(&self) -> &str {
        match self {
            Type_::Adult => "Adult - $8.00",
            Type_::Senior => "Adult - $5.00",
            Type_::ChildUnderThirteen => "Child (6-12) - $3.00",
            Type_::ChildUnderSix => "Child (Under 6) - Free",
            Type_::PfspMember => "PFSP Member - Free"
        }
    }
}