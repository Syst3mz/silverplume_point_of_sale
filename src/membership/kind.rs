use strum::{Display, VariantArray};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Default, VariantArray, Display)]
pub enum Kind {
    #[default]
    #[strum(serialize = "Family - $40.00")]
    Family,
    #[strum(serialize = "Individual - $25.00")]
    Individual,
    #[strum(serialize = "Senior Family (60+) - $15.00")]
    SeniorFamily,
    #[strum(serialize = "Senior Individual (60+) - $25.00")]
    SeniorIndividual,
    #[strum(serialize = "Life Member - $750.00")]
    LifetimeMember
}

impl Kind {
    pub fn price(&self) -> f32 {
        match self {
            Kind::Family => 40.0,
            Kind::Individual => 25.0,
            Kind::SeniorFamily => 15.0,
            Kind::SeniorIndividual => 25.0,
            Kind::LifetimeMember => 750.0,
        }
    }
}