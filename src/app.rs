use chrono::{Duration, Local};
use iced::advanced::Widget;
use iced::alignment::Horizontal;
use iced::Element;
use iced::widget::{container, horizontal_rule, scrollable, text};
use crate::{HEADER_SIZE, RULE_HEIGHT, TEXT_SIZE};
use crate::database::Database;
use crate::get_payment_method::GetPaymentMethod;
use crate::payment_method::PaymentMethod;
use crate::sale_screen::SaleScreen;
use crate::transaction_record::{Hour, TransactionKind};



pub struct App {
    sale_screen: SaleScreen,
    database: Database,
    error: Option<anyhow::Error>,
}

type SaleMessage = crate::sale_screen::Message;
#[derive(Debug, Clone)]
pub enum Message {
    SaleMessage(SaleMessage)
}


impl App {
    fn handle_sale_message(&mut self, message: SaleMessage) {
        // For the love of all that is good, do the sale screen update AFTER the transaction is written to the database.
        let error = match message.clone() {
            //SaleMessage::AddAdmission => self.database.store_in_db(self.sale_screen.admission()),
            /*SaleMessage::AddDonation => self.database.store_in_db(self.sale_screen.donation()),
            SaleMessage::AddMembership => self.database.store_in_db(self.sale_screen.membership()),
            SaleMessage::AddGiftShopSale => self.database.store_in_db(self.sale_screen.gift_shop_sale()),*/
            _ => {Ok(())}
        };
        
        if let Err(error) = error {
            println!("{:?}", error);
            self.error = Some(error);
        } else {
            self.error = None;
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
        let mut grid = iced::widget::column![].spacing(RULE_HEIGHT);
        let mut values = values.into_iter();

        let mut row = iced::widget::Row::new().spacing(RULE_HEIGHT);
        while let Some((name, value)) = values.next() {
            row = row.push(Self::summary_box(name, value));
            if row.children().len() == 5 {
                grid = grid.push(row);
                row = iced::widget::Row::new().spacing(RULE_HEIGHT);
            }
        }

        if !row.children().is_empty() {
            grid = grid.push(row);
        }


        iced::widget::column![
            text(header.as_ref().to_string()).size(HEADER_SIZE),
            horizontal_rule(RULE_HEIGHT),
            grid
        ].padding(RULE_HEIGHT).align_x(Horizontal::Center).into()
    }
    

    fn summary(&self) -> Element<Message> {
        type T = TransactionKind;
        type At = crate::admission::kind::Kind;
        type Mk = crate::membership::kind::Kind;
        type Pm = PaymentMethod;
        let now = Local::now();
        iced::widget::column![
            /*self.summary_row("Daily Summary", [
                ("Total Attendance", self.database.todays_transactions_of_kind(T::Admission).map(|x| x.quantity as u32).sum::<u32>().to_string()),
                ("Admissions Revenue", self.sum_todays_transactions_of_kind(T::Admission)),
                ("Total Donations", self.sum_todays_transactions_of_kind(T::Donation)),
                ("Membership Sales", self.sum_todays_transactions_of_kind(T::Membership)),
                ("Gift Shop Sales", self.sum_todays_transactions_of_kind(T::GiftShopSale)),
                ("Sales Tax Collected", format!("${:.2}", self.database.gift_shop_sales.iter().filter(|x| (now - x.date()) <= Duration::days(1)).map(|x| x.compute_tax()).sum::<f32>())),
                ("Total Daily Revenue", format!("${:.2}", self.database.todays_transactions().map(|x| x.total_cost).sum::<f32>())),
            ]),
            self.summary_row("Monthly Payments Breakdown", [
                ("Cash - Admissions", format!("${:.2}", Self::filter_by_payment_method(&self.database.admissions, Pm::Cash).map(|x| x.compute_total_cost()).sum::<f32>())),
                ("Credit Card - Admissions", format!("${:.2}", Self::filter_by_payment_method(&self.database.admissions, Pm::CreditCard).map(|x| x.compute_total_cost()).sum::<f32>().to_string())),
                ("Free - Admissions", self.database.admissions.iter().filter(|x| !x.needs_payment()).count().to_string()),
                ("Cash - Donations", format!("${:.2}", Self::filter_by_payment_method(&self.database.donations, Pm::Cash).map(|x| x.amount()).sum::<f32>())),
                ("Credit Card - Donations", format!("${:.2}", Self::filter_by_payment_method(&self.database.donations, Pm::CreditCard).map(|x| x.amount()).sum::<f32>())),
                ("Cash - Memberships", format!("${:.2}", Self::filter_by_payment_method(&self.database.memberships, Pm::Cash).map(|x| x.compute_total_cost()).sum::<f32>())),
                ("Credit Card - Memberships", format!("${:.2}", Self::filter_by_payment_method(&self.database.memberships, Pm::CreditCard).map(|x| x.compute_total_cost()).sum::<f32>())),
                ("Cash - Shop Sales", format!("${:.2}", Self::filter_by_payment_method(&self.database.gift_shop_sales, Pm::Cash).map(|x| x.compute_total_cost()).sum::<f32>())),
                ("Credit Card - Shop Sales", format!("${:.2}", Self::filter_by_payment_method(&self.database.gift_shop_sales, Pm::CreditCard).map(|x| x.compute_total_cost()).sum::<f32>())),
                ("Total Cash", self.total_by_payment_method(PaymentMethod::Cash)),
                ("Total Credit Card", self.total_by_payment_method(PaymentMethod::CreditCard)),
            ]),
            self.summary_row("Monthly Admission Breakdown", [
                ("Adults", self.sum_admission_type(At::Adult)),
                ("Seniors", self.sum_admission_type(At::Adult)),
                ("Children (6-12)", self.sum_admission_type(At::ChildUnderThirteen)),
                ("Children (Under 6)", self.sum_admission_type(At::ChildUnderSix)),
                ("PFSP Members", self.sum_admission_type(At::PfspMember)),
            ]),
            self.summary_row("Monthly Membership Sales Breakdown", [
                ("Family", self.sum_membership_sales_type(Mk::Family)),
                ("Individual", self.sum_membership_sales_type(Mk::Individual)),
                ("Senior Family", self.sum_membership_sales_type(Mk::SeniorFamily)),
                ("Senior Individual", self.sum_membership_sales_type(Mk::SeniorIndividual)),
                ("Lifetime Member", self.sum_membership_sales_type(Mk::LifetimeMember)),
            ]),
            self.summary_row("Monthly Admission Breakdown", [
                ("11-12pm", self.sum_hourly_attendance(Hour::ElevenToTwelve)),
                ("12-1pm", self.sum_hourly_attendance(Hour::TwelveToOne)),
                ("1-2pm", self.sum_hourly_attendance(Hour::OneToTwo)),
                ("2-3pm", self.sum_hourly_attendance(Hour::TwoToThree)),
                ("3-4pm", self.sum_hourly_attendance(Hour::ThreeToFour)),
            ])*/
        ].spacing(RULE_HEIGHT).into()
    }

    pub fn view(&self) -> Element<Message> {
        scrollable(iced::widget::column![
            self.sale_screen.view().map(Message::SaleMessage),
            self.summary(),

        ].spacing(2 * RULE_HEIGHT)).into()

    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            sale_screen: Default::default(),
            database: Default::default(),
            error: None,
        }
    }
}