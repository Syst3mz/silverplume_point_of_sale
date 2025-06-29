use iced::Element;
use iced::widget::{button, row};
use crate::view::admission::Admission;
use crate::view::donation::Donation;
use crate::view::gift_shop_sale::GiftShopSale;
use crate::view::membership::Membership;
use crate::RULE_HEIGHT;
pub struct SaleScreen {
    admission: Admission,
    donation: Donation,
    membership: Membership,
    gift_shop_sale: GiftShopSale,
}

#[derive(Debug, Clone)]
pub enum Message {
    Admission(crate::view::admission::Message),
    AddAdmission,
    Donation(crate::view::donation::Message),
    AddDonation,
    Membership(crate::view::membership::Message),
    AddMembership,
    GiftShopSale(crate::view::gift_shop_sale::Message),
    AddGiftShopSale,
}

impl SaleScreen {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Admission(a) => self.admission.update(a),
            Message::Donation(d) => self.donation.update(d),
            Message::Membership(m) => self.membership.update(m),
            Message::GiftShopSale(g) => self.gift_shop_sale.update(g),
            
            Message::AddAdmission => self.admission = Default::default(),
            Message::AddDonation => self.donation = Default::default(),
            Message::AddMembership => self.membership = Default::default(),
            Message::AddGiftShopSale => self.gift_shop_sale = Default::default(),
        }
    }
    pub fn view(&self) -> Element<Message> {
        iced::widget::column![
            row![
                iced::widget::column![
                    self.admission.view().map(|x| Message::Admission(x)),
                    button("Add Admission").on_press_maybe(self.admission.is_valid().then(|| Message::AddAdmission)),
                ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
                iced::widget::column![
                    self.donation.view().map(|x| Message::Donation(x)),
                    button("Add Donation").on_press_maybe(self.donation.is_valid().then(|| Message::AddDonation)),
                ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
            ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
            row![
                iced::widget::column![
                    self.membership.view().map(|x| Message::Membership(x)),
                    button("Add Membership").on_press_maybe(self.membership.is_valid().then(|| Message::AddMembership)),
                ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
                iced::widget::column![
                    self.gift_shop_sale.view().map(|x| Message::GiftShopSale(x)),
                    button("Add Sale").on_press_maybe(self.gift_shop_sale.is_valid().then(|| Message::AddGiftShopSale)),
                ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
            ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
        ].spacing(RULE_HEIGHT).into()
    }

    pub fn admission(&self) -> &Admission {
        &self.admission
    }
    pub fn donation(&self) -> &Donation {
        &self.donation
    }
    pub fn membership(&self) -> &Membership {
        &self.membership
    }
    pub fn gift_shop_sale(&self) -> &GiftShopSale {
        &self.gift_shop_sale
    }
}

impl Default for SaleScreen {
    fn default() -> Self {

        Self {
            admission: Default::default(),
            donation: Default::default(),
            membership: Default::default(),
            gift_shop_sale: Default::default(),
        }
    }
}