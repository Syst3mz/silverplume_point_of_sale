use strum::{Display, VariantArray};
use crate::as_description::AsDescription;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, VariantArray, Display, Default)]
pub enum Kind {
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

impl Kind {
    pub fn is_free(&self) -> bool {
        self.cost() == 0.0
    }

    pub fn cost(&self) -> f32 {
        match self {
            Kind::Adult => 8.0,
            Kind::Senior => 5.0,
            Kind::ChildUnderThirteen => 3.0,
            Kind::ChildUnderSix | Kind::PfspMember => 0.0
        }
    }
}

impl AsDescription for Kind {
    fn as_description(&self) -> &str {
        match self {
            Kind::Adult => "Adult - $8.00",
            Kind::Senior => "Adult - $5.00",
            Kind::ChildUnderThirteen => "Child (6-12) - $3.00",
            Kind::ChildUnderSix => "Child (Under 6) - Free",
            Kind::PfspMember => "PFSP Member - Free"
        }
    }
}