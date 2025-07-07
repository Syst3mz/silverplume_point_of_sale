use sqlite::Value;
use strum::{Display, EnumString, VariantArray};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Default, VariantArray, Display, EnumString)]
pub enum Kind {
    #[default]
    #[strum(serialize = "Family")]
    Family,
    #[strum(serialize = "Individual")]
    Individual,
    #[strum(serialize = "Senior Family (60+)")]
    SeniorFamily,
    #[strum(serialize = "Senior Individual (60+)")]
    SeniorIndividual,
    #[strum(serialize = "Life Member")]
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

impl TryFrom<&Value> for Kind {
    type Error = sqlite::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let Value::String(value) = value else {
            return Err(sqlite::Error {
                code: None,
                message: Some("Value is not a string, and must be.".to_string()),
            })
        };

        Kind::try_from(value.as_str()).map_err(|_| sqlite::Error {
            code: None,
            message: Some("Unable to convert string to membership kind.".to_string()),
        })
    }
}