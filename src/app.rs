use iced::alignment::Horizontal;
use iced::Element;
use iced::widget::{container, horizontal_rule, row, text};
use crate::{HEADER_SIZE, RULE_HEIGHT, TEXT_SIZE};
use crate::database::Database;
use crate::sale_screen::SaleScreen;
use crate::transaction_record::TransactionKind;



pub struct App {
    sale_screen: SaleScreen,
    database: Database
}

type SaleMessage = crate::sale_screen::Message;
#[derive(Debug, Clone)]
pub enum Message {
    SaleMessage(SaleMessage)
}

impl App {
    fn handle_sale_message(&mut self, message: SaleMessage) {
        // For the love of all that is good, do the sale screen update AFTER the transaction is written to the database.
        match message.clone() {
            SaleMessage::AddAdmission => { 
                self.database.add_admission(self.sale_screen.admission().clone())
            }
            SaleMessage::AddDonation => {
                self.database.add_donation(self.sale_screen.donation().clone())
            }
            SaleMessage::AddMembership => {
                self.database.add_membership(self.sale_screen.membership().clone())
            }
            SaleMessage::AddGiftShopSale => {
                self.database.add_gift_shop_sale(self.sale_screen.gift_shop_sale().clone())
            }
            _ => {}
        }
        self.sale_screen.update(message);
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SaleMessage(s) => self.handle_sale_message(s)
        }
    }

    fn summary_box(name: impl AsRef<str>, value: impl AsRef<str>) -> Element<'static, Message> {
        let name = name.as_ref().to_string();
        let value = value.as_ref().to_string();
        container(iced::widget::column![
            text(name).align_x(Horizontal::Center),
            text(value).size(TEXT_SIZE + 4).align_x(Horizontal::Center)
        ].spacing(RULE_HEIGHT).align_x(Horizontal::Center).padding(RULE_HEIGHT))
            .style(container::rounded_box)
            .into()
    }

    fn summary_row(&self, header: impl AsRef<str>, values: impl IntoIterator<Item=(impl AsRef<str>, impl AsRef<str>)>) -> Element<'static, Message> {
        iced::widget::column![
            text(header.as_ref().to_string()).size(HEADER_SIZE),
            horizontal_rule(RULE_HEIGHT),
            row(values.into_iter().map(|x| Self::summary_box(x.0, x.1))).spacing(RULE_HEIGHT)
        ].into()
    }

    fn summary(&self) -> Element<Message> {
        type T = TransactionKind;
        iced::widget::column![
            self.summary_row("Daily Summary", [
                ("Total Attendance", self.database.todays_transactions_of_kind(T::Admission).map(|x| x.quantity as u32).sum::<u32>().to_string()),
                ("Admissions Revenue", self.database.todays_transactions_of_kind(T::Admission).map(|x| x.amount).sum::<f32>().to_string()),
                ("Total Donations", self.database.todays_transactions_of_kind(T::Donation).map(|x| x.amount).sum::<f32>().to_string()),
                ("Membership Sales", self.database.todays_transactions_of_kind(T::Membership).map(|x| x.amount).sum::<f32>().to_string()),
                ("Gift Shop Sales", self.database.todays_transactions_of_kind(T::GiftShopSale).map(|x| x.amount).sum::<f32>().to_string()),
                ("Membership Sales", self.database.todays_transactions_of_kind(T::Membership).map(|x| x.amount).sum::<f32>().to_string()),
                ("Sales Tax Collected", self.database.gift_shop_sales.iter().map(|x| x.compute_tax()).sum::<f32>().to_string()),
                ("Total Daily Revenue", self.database.todays_transactions().map(|x| x.amount).sum::<f32>().to_string()),
            ]),
        ].spacing(RULE_HEIGHT).into()
    }

    pub fn view(&self) -> Element<Message> {
        iced::widget::column![
            self.sale_screen.view().map(Message::SaleMessage),
            self.summary(),
            
        ].spacing(2 * RULE_HEIGHT).into()

    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            sale_screen: Default::default(),
            database: Default::default(),
        }
    }
}