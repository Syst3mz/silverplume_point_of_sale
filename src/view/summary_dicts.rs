use indexmap::IndexMap;
use serde::Serialize;
use crate::database::Database;
use crate::model::has_payment_method::HasPaymentMethod;
use crate::model::payment_method::PaymentMethod;
use crate::model::has_total_cost::HasTotalCost;

type Dictionary = IndexMap<&'static str, String>;
#[derive(Debug, Clone, Serialize)]
pub struct SummaryDicts {
    pub summary: Dictionary,
    pub payments: Dictionary,
    pub admissions: Dictionary,
    pub memberships: Dictionary,
}

fn total_money_by_payment_method(database: &Database, payment_method: PaymentMethod) -> f32 {
    let mut total = 0.0;
    database.daily_admissions().iter().filter(|x| x.matches_payment_method(payment_method)).for_each(|x| {
        total += x.total_cost()
    });

    database.daily_memberships().iter().filter(|x| x.matches_payment_method(payment_method)).for_each(|x| {
        total += x.total_cost()
    });

    database.daily_gift_shop_sales().iter().filter(|x| x.matches_payment_method(payment_method)).for_each(|x| {
        total += x.total_cost()
    });

    database.daily_donations().iter().filter(|x| x.matches_payment_method(payment_method)).for_each(|x| {
        total += x.total_cost()
    });

    total
}

impl SummaryDicts {
    pub fn new(database: &Database) -> Self {
        use crate::view::adapters::*;

        type At = crate::model::admission::kind::Kind;
        type Mk = crate::model::membership::kind::Kind;
        type Pm = PaymentMethod;

        Self {
            summary: IndexMap::from([
                ("Total Attendance", database.daily_admissions().iter().map(|x| x.quantity as u32).sum::<u32>().to_string()),
                ("Admissions Revenue", ff("$", database.daily_admissions().total_cost())),
                ("Total Donations", ff("$", database.daily_donations().total_cost())),
                ("Membership Sales", ff("$",database.daily_memberships().total_cost())),
                ("Gift Shop Sales", ff("$",database.daily_gift_shop_sales().total_cost())),
                ("Sales Tax Collected", format!("${:.2}", database.daily_gift_shop_sales().iter().map(|x| x.compute_tax()).sum::<f32>())),
                ("Total Daily Revenue", ff("$", database.daily_transactions().total_cost())),
            ]),
            payments: IndexMap::from([
                ("Cash - Admissions", ff("$", filter_by_payment_and_sum(database.daily_admissions(), Pm::Cash))),
                ("Credit Card - Admissions", ff("$", filter_by_payment_and_sum(database.daily_admissions(), Pm::CreditCard))),
                ("Free - Admissions", database.daily_admissions().len().to_string()),
                ("Cash - Donations", ff("$", filter_by_payment_and_sum(database.daily_donations(), Pm::Cash))),
                ("Credit Card - Donations", ff("$", filter_by_payment_and_sum(database.daily_donations(), Pm::CreditCard))),
                ("Cash - Memberships", ff("$", filter_by_payment_and_sum(database.daily_memberships(), Pm::Cash))),
                ("Credit Card - Memberships", ff("$", filter_by_payment_and_sum(database.daily_memberships(), Pm::CreditCard))),
                ("Cash - Shop Sales", ff("$", filter_by_payment_and_sum(database.daily_gift_shop_sales(), Pm::Cash))),
                ("Credit Card - Shop Sales", ff("$", filter_by_payment_and_sum(database.daily_gift_shop_sales(), Pm::CreditCard))),
                ("Total Cash", ff("$", total_money_by_payment_method(database, Pm::Cash))),
                ("Total Credit Card", ff("$", total_money_by_payment_method(database, Pm::CreditCard))),
            ]),
            admissions: IndexMap::from([
                ("Adults", sum_over_admission_kind(database.daily_admissions(), At::Adult).to_string()),
                ("Seniors", sum_over_admission_kind(database.daily_admissions(), At::Senior).to_string()),
                ("Children (6-12)", sum_over_admission_kind(database.daily_admissions(), At::ChildUnderThirteen).to_string()),
                ("Children (Under 6)", sum_over_admission_kind(database.daily_admissions(), At::ChildUnderSix).to_string()),
                ("PFSP Members", sum_over_admission_kind(database.daily_admissions(), At::PfspMember).to_string()),
            ]),
            memberships: IndexMap::from([
                ("Family", sum_over_membership_sale(database.daily_memberships(), Mk::Family).to_string()),
                ("Individual", sum_over_membership_sale(database.daily_memberships(), Mk::Individual).to_string()),
                ("Senior Family", sum_over_membership_sale(database.daily_memberships(), Mk::SeniorFamily).to_string()),
                ("Senior Individual", sum_over_membership_sale(database.daily_memberships(), Mk::SeniorIndividual).to_string()),
                ("Lifetime Member", sum_over_membership_sale(database.daily_memberships(), Mk::LifetimeMember).to_string()),
            ]),
        }
    }
}