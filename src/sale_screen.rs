use iced::Element;
use iced::widget::{button, row};
use crate::admission::Admission;
use crate::as_transaction_record::AsTransactionRecord;
use crate::donation::Donation;
use crate::gift_shop_sale::GiftShopSale;
use crate::membership::Membership;
use crate::RULE_HEIGHT;
use crate::transaction_record::TransactionRecord;

const DATABASE_FILE_NAME: &str = "point_of_sale_db.json";
pub struct SaleScreen {
    admission: Admission,
    donation: Donation,
    membership: Membership,
    gift_shop_sale: GiftShopSale,
}

#[derive(Debug, Clone)]
pub enum Message {
    Admission(crate::admission::Message),
    AddAdmission,
    Donation(crate::donation::Message),
    AddDonation,
    Membership(crate::membership::Message),
    AddMembership,
    GiftShopSale(crate::gift_shop_sale::Message),
    AddGiftShopSale,
}

impl SaleScreen {
    pub fn add_and_clear<T: Default+AsTransactionRecord>(transactions: &mut Vec<TransactionRecord>, to_clear: &mut T) {
        transactions.push(to_clear.as_transaction_record());
        *to_clear = Default::default();
        write_db(&transactions);
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Admission(a) => self.admission.update(a),
            Message::AddAdmission => Self::add_and_clear(&mut self.transactions, &mut self.admission),
            Message::Donation(d) => self.donation.update(d),
            Message::AddDonation => Self::add_and_clear(&mut self.transactions, &mut self.donation),
            Message::Membership(m) => self.membership.update(m),
            Message::AddMembership => Self::add_and_clear(&mut self.transactions, &mut self.membership),
            Message::GiftShopSale(g) => self.gift_shop_sale.update(g),
            Message::AddGiftShopSale => Self::add_and_clear(&mut self.transactions, &mut self.gift_shop_sale)
        }
    }
    pub fn view(&self) -> Element<Message> {
        iced::widget::column![
            row![
                iced::widget::column![
                    self.admission.view().map(|x| Message::Admission(x)),
                    button("Add Admission").on_press(Message::AddAdmission),
                ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
                iced::widget::column![
                    self.donation.view().map(|x| Message::Donation(x)),
                    button("Add Donation").on_press(Message::AddDonation),
                ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
            ].spacing(RULE_HEIGHT),
            row![
                iced::widget::column![
                    self.membership.view().map(|x| Message::Membership(x)),
                    button("Add Membership").on_press(Message::AddMembership),
                ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
                iced::widget::column![
                    self.gift_shop_sale.view().map(|x| Message::GiftShopSale(x)),
                    button("Add Sale").on_press(Message::AddGiftShopSale),
                ].spacing(RULE_HEIGHT).padding(RULE_HEIGHT),
            ].spacing(RULE_HEIGHT)
        ].spacing(RULE_HEIGHT).into()
    }
}

fn read_database() -> Option<Vec<TransactionRecord>> {
    let database_file = std::fs::read_to_string(DATABASE_FILE_NAME).ok()?;
    let database = serde_json::from_str(&database_file);

    database.ok()
}

fn write_db(transactions: &Vec<TransactionRecord>) {
    let json = serde_json::to_string_pretty(transactions);
    let _ = std::fs::write(DATABASE_FILE_NAME, json.unwrap());
}

impl Default for SaleScreen {
    fn default() -> Self {
        let transactions = read_database().unwrap_or(vec![]);

        Self {
            admission: Default::default(),
            donation: Default::default(),
            membership: Default::default(),
            gift_shop_sale: Default::default(),
            transactions,
        }
    }
}