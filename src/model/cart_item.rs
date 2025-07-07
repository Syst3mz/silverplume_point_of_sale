use std::fmt::Display;
use crate::model::admission::Admission;
use crate::model::donation::Donation;
use crate::model::membership::Membership;
use crate::model::gift_shop_sale::GiftShopSale;
use crate::model::has_total_cost::HasTotalCost;

#[derive(Debug, Clone)]
pub enum CartItem {
    Admission(Admission),
    Membership(Membership),
    Donation(Donation),
    GiftShopSale(GiftShopSale)
}

impl Display for CartItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CartItem::Admission(x) => x.to_string(),
            CartItem::Membership(x) => x.to_string(),
            CartItem::Donation(x) => x.to_string(),
            CartItem::GiftShopSale(x) => x.to_string(),
        })
    }
}

impl From<Admission> for CartItem {
    fn from(x: Admission) -> Self {
        CartItem::Admission(x)
    }
}

impl From<Membership> for CartItem {
    fn from(x: Membership) -> Self {
        CartItem::Membership(x)
    }
}

impl From<Donation> for CartItem {
    fn from(x: Donation) -> Self {
        CartItem::Donation(x)
    }
}

impl From<GiftShopSale> for CartItem {
    fn from(x: GiftShopSale) -> Self {
        CartItem::GiftShopSale(x)
    }
}

impl HasTotalCost for CartItem {
    fn total_cost(&self) -> f32 {
        match self {
            CartItem::Admission(a) => a.total_cost(),
            CartItem::Membership(m) => m.total_cost(),
            CartItem::Donation(d) => d.total_cost(),
            CartItem::GiftShopSale(g) => g.total_cost(),
        }
    }
}