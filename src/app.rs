use iced::advanced::Widget;
use iced::alignment::Horizontal;
use iced::Element;
use iced::widget::{container, horizontal_rule, scrollable, text};
use crate::{HEADER_SIZE, RULE_HEIGHT, TEXT_SIZE};
use crate::database::Database;
use crate::database::database_object::CanBuildObjectMapper;
use crate::model::admission::Admission;
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::date_time_wrapper::WrapInDateTime;
use crate::model::has_payment_method::HasPaymentMethod;
use crate::model::has_total_cost::HasTotalCost;
use crate::model::payment_method::PaymentMethod;
use crate::sale_screen::SaleScreen;
use crate::to_model::ToModel;
use crate::model::membership::Membership;

pub struct App {
    sale_screen: SaleScreen,
    database: Database,
    error: Option<anyhow::Error>,
}

type SaleMessage = crate::sale_screen::Message;
#[derive(Debug, Clone)]
pub enum Message {
    SaleMessage(SaleMessage),
}

impl App {    
    fn transactionify_and_insert<S, T>(&mut self, object: S) -> anyhow::Result<()> where
        T: CanBuildObjectMapper +AsTransactionRecord+WrapInDateTime,
        S: ToModel<ModelType=T> 
    {
        let object = object.to_model()?;
        self.database.insert(object.as_transaction_record().wrapped_in_date_time())?;
        self.database.insert(object.wrapped_in_date_time())
    }
    
    fn handle_sale_message(&mut self, message: SaleMessage) {
        // For the love of all that is good, do the sale screen update AFTER the transaction is written to the database.
        let error = match message.clone() {
            SaleMessage::AddAdmission => self.transactionify_and_insert(self.sale_screen.admission().clone()),
            SaleMessage::AddDonation => self.transactionify_and_insert(self.sale_screen.donation().clone()),
            SaleMessage::AddMembership => self.transactionify_and_insert(self.sale_screen.membership().clone()),
            SaleMessage::AddGiftShopSale => self.transactionify_and_insert(self.sale_screen.gift_shop_sale().clone()),
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
            Message::SaleMessage(s) => self.handle_sale_message(s),
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

    fn total_money_by_payment_method(&self, payment_method: PaymentMethod) -> f32 {
        let mut total = 0.0;
        self.database.daily_admissions().iter().filter(|x| x.matches_payment_method(payment_method)).for_each(|x| {
            total += x.total_cost()
        });

        self.database.daily_memberships().iter().filter(|x| x.matches_payment_method(payment_method)).for_each(|x| {
            total += x.total_cost()
        });

        self.database.daily_gift_shop_sales().iter().filter(|x| x.matches_payment_method(payment_method)).for_each(|x| {
            total += x.total_cost()
        });

        self.database.daily_donations().iter().filter(|x| x.matches_payment_method(payment_method)).for_each(|x| {
            total += x.total_cost()
        });

        total
    }

    fn summary(&self) -> Element<Message> {
        type At = crate::model::admission::kind::Kind;
        type Mk = crate::model::membership::kind::Kind;
        type Pm = PaymentMethod;
        use crate::model::has_total_cost::HasTotalCost;

        fn ff(prefix: impl AsRef<str>, float: f32) -> String {
            let prefix = prefix.as_ref();
            if (0.0 - float).abs() < 0.000001 {
                format!("{prefix}0")
            } else {
                format!("{prefix}{:.2}", float)
            }
        }

        fn filter_by_payment_and_sum<'a, T>(iter: impl IntoIterator<Item=&'a T>, method: Pm) -> f32 where
        T: HasPaymentMethod+HasTotalCost + 'a
        {
            iter.into_iter()
                .filter(|x| x.matches_payment_method(method))
                .map(|x| x.total_cost())
                .sum()
        }

        fn sum_over_admission_kind(admissions: &Vec<Admission>, kind: At) -> u32 {
            admissions.into_iter().filter_map(|x| (x.kind == kind).then_some(x.quantity as u32)).sum()
        }

        fn sum_over_membership_sale(memberships: &Vec<Membership>, kind: Mk) -> u32 {
            memberships.into_iter().filter_map(|x| x.matches_type(kind).then_some(x.quantity as u32)).sum()
        }


        iced::widget::column![
            self.summary_row("Daily Summary", [
                ("Total Attendance", self.database.daily_admissions().iter().map(|x| x.quantity as u32).sum::<u32>().to_string()),
                ("Admissions Revenue", ff("$", self.database.daily_admissions().total_cost())),
                ("Total Donations", ff("$",self.database.daily_donations().total_cost())),
                ("Membership Sales", ff("$",self.database.daily_memberships().total_cost())),
                ("Gift Shop Sales", ff("$",self.database.daily_gift_shop_sales().total_cost())),
                ("Sales Tax Collected", format!("${:.2}", self.database.daily_gift_shop_sales().iter().map(|x| x.compute_tax()).sum::<f32>())),
                ("Total Daily Revenue", ff("$", self.database.daily_transactions().total_cost())),
            ]),
            self.summary_row("Daily Payments Breakdown", [
                ("Cash - Admissions", ff("$", filter_by_payment_and_sum(self.database.daily_admissions(), Pm::Cash))),
                ("Credit Card - Admissions", ff("$", filter_by_payment_and_sum(self.database.daily_admissions(), Pm::CreditCard))),
                ("Free - Admissions", self.database.daily_admissions().len().to_string()),
                ("Cash - Donations", ff("$", filter_by_payment_and_sum(self.database.daily_donations(), Pm::Cash))),
                ("Credit Card - Donations", ff("$", filter_by_payment_and_sum(self.database.daily_donations(), Pm::CreditCard))),
                ("Cash - Memberships", ff("$", filter_by_payment_and_sum(self.database.daily_memberships(), Pm::Cash))),
                ("Credit Card - Memberships", ff("$", filter_by_payment_and_sum(self.database.daily_memberships(), Pm::CreditCard))),
                ("Cash - Shop Sales", ff("$", filter_by_payment_and_sum(self.database.daily_gift_shop_sales(), Pm::Cash))),
                ("Credit Card - Shop Sales", ff("$", filter_by_payment_and_sum(self.database.daily_gift_shop_sales(), Pm::CreditCard))),
                ("Total Cash", ff("$", self.total_money_by_payment_method(Pm::Cash))),
                ("Total Credit Card", ff("$", self.total_money_by_payment_method(Pm::CreditCard))),
            ]),
            self.summary_row("Daily Admission Breakdown", [
                ("Adults", sum_over_admission_kind(self.database.daily_admissions(), At::Adult).to_string()),
                ("Seniors", sum_over_admission_kind(self.database.daily_admissions(), At::Senior).to_string()),
                ("Children (6-12)", sum_over_admission_kind(self.database.daily_admissions(), At::ChildUnderThirteen).to_string()),
                ("Children (Under 6)", sum_over_admission_kind(self.database.daily_admissions(), At::ChildUnderSix).to_string()),
                ("PFSP Members", sum_over_admission_kind(self.database.daily_admissions(), At::PfspMember).to_string()),
            ]),
            self.summary_row("Daily Membership Sales Breakdown", [
                ("Family", sum_over_membership_sale(self.database.daily_memberships(), Mk::Family).to_string()),
                ("Individual", sum_over_membership_sale(self.database.daily_memberships(), Mk::Individual).to_string()),
                ("Senior Family", sum_over_membership_sale(self.database.daily_memberships(), Mk::SeniorFamily).to_string()),
                ("Senior Individual", sum_over_membership_sale(self.database.daily_memberships(), Mk::SeniorIndividual).to_string()),
                ("Lifetime Member", sum_over_membership_sale(self.database.daily_memberships(), Mk::LifetimeMember).to_string()),
            ]),
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
            database: Database::new(),
            error: None,
        }
    }
}