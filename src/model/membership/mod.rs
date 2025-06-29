use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::membership::kind::Kind;
use crate::model::payment_method::PaymentMethod;
use crate::model::transaction_record::{TransactionKind, TransactionRecord};

pub mod kind;
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Membership {
    kind: Kind,
    payment_method: PaymentMethod,
    pub quantity: u16
}


impl Membership {
    pub fn matches_type(&self, kind: Kind) -> bool {
        self.kind == kind
    }
    
    pub fn compute_total_cost(&self) -> f32 {
        self.quantity as f32 * self.kind.price()
    }
}

impl Default for Membership {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            payment_method: Default::default(),
            quantity: 1,
        }
    }
}

impl AsTransactionRecord for Membership {
    fn as_transaction_record(&self) -> TransactionRecord {
        TransactionRecord::new (
            TransactionKind::Membership,
            self.kind.to_string(),
            self.quantity,
            self.compute_total_cost()
        )
    }
}