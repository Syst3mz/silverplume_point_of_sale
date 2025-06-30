pub mod kind;

use crate::as_description::AsDescription;
use crate::database::database_object::DatabaseObject;
use crate::database::object_mapper::ObjectMapper;
use crate::model::admission::kind::Kind;
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::date_time_wrapper::WrapInDateTime;
use crate::model::get_payment_method::GetPaymentMethod;
use crate::model::payment_method::PaymentMethod;
use crate::model::transaction_record::{TransactionKind, TransactionRecord};

#[derive(Debug, Clone, Copy)]
pub struct Admission {
    pub kind: Kind,
    payment_method: Option<PaymentMethod>,
    pub quantity: u16,
}

impl Admission {
    pub fn new(kind: Kind, payment_method: Option<PaymentMethod>, quantity: u16) -> Admission {
        Self {
            kind,
            payment_method,
            quantity,
        }
    }
    pub fn needs_payment(&self) -> bool {
        !self.kind.is_free()
    }

    pub fn matches_admission_type(&self, kind: Kind) -> bool {
        self.kind == kind
    }

    pub fn compute_total_cost(&self) -> f32 {
        self.quantity as f32 * self.kind.cost()
    }
}

impl AsTransactionRecord for Admission {
    fn as_transaction_record(&self) -> TransactionRecord {
        TransactionRecord::new(
            TransactionKind::Admission,
            self.kind.as_description().to_string(),
            self.quantity,
            self.compute_total_cost()
        )
    }
}

impl GetPaymentMethod for Admission {
    fn get_payment_method(&self) -> Option<PaymentMethod> {
        self.payment_method.clone()
    }
}

impl WrapInDateTime for Admission {}
impl DatabaseObject for Admission {
    fn build_object_mapper(&self) -> ObjectMapper {
        ObjectMapper::new("admissions")
            .add_field("kind", self.kind.as_description().to_string())
            .add_field("payment_method", self.payment_method)
            .add_field("quantity", self.quantity as i32)
    }
}
impl Default for Admission {
    fn default() -> Admission {
        Self {
            kind: Default::default(),
            payment_method: None,
            quantity: 0,
        }
    }
}