use crate::model::admission::Admission;
use crate::model::has_payment_method::HasPaymentMethod;
use crate::model::has_total_cost::HasTotalCost;
use crate::model::membership::Membership;

pub fn ff(prefix: impl AsRef<str>, float: f32) -> String {
    let prefix = prefix.as_ref();
    if (0.0 - float).abs() < 0.000001 {
        format!("{prefix}0")
    } else {
        format!("{prefix}{:.2}", float)
    }
}

pub fn filter_by_payment_and_sum<'a, T>(iter: impl IntoIterator<Item=&'a T>, method: crate::model::payment_method::PaymentMethod) -> f32 where
    T: HasPaymentMethod+HasTotalCost + 'a
{
    iter.into_iter()
        .filter(|x| x.matches_payment_method(method))
        .map(|x| x.total_cost())
        .sum()
}

pub fn sum_over_admission_kind(admissions: &Vec<Admission>, kind: crate::model::admission::kind::Kind) -> u32 {
    admissions.into_iter().filter_map(|x| (x.kind == kind).then_some(x.quantity as u32)).sum()
}

pub fn sum_over_membership_sale(memberships: &Vec<Membership>, kind: crate::model::membership::kind::Kind) -> u32 {
    memberships.into_iter().filter_map(|x| x.matches_type(kind).then_some(x.quantity as u32)).sum()
}