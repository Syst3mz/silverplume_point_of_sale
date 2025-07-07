use chrono::{Datelike, Local};
use iced::advanced::Widget;
use iced::alignment::Horizontal;
use iced::Element;
use iced::widget::{button, container, horizontal_rule, scrollable, text};
use crate::{HEADER_SIZE, RULE_HEIGHT, TEXT_SIZE};
use crate::app::Message::RenderDailyReport;
use crate::database::Database;
use crate::database::database_object::CanBuildObjectMapper;
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::date_time_wrapper::WrapInDateTime;
use crate::sale_screen::SaleScreen;
use crate::to_model::ToModel;
use crate::view::summary_dicts::SummaryDicts;

pub struct App {
    sale_screen: SaleScreen,
    database: Database,
    error: Option<anyhow::Error>,
}

type SaleMessage = crate::sale_screen::Message;
#[derive(Debug, Clone)]
pub enum Message {
    SaleMessage(SaleMessage),
    RenderDailyReport
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
            Message::RenderDailyReport => {
                let now = Local::now();
                let _ =std::fs::write(
                    format!("{}{}{}_report.html", now.year(), now.month(), now.day()),
                    self.database.render_to_html()
                );
            }
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
        let summaries = SummaryDicts::new(&self.database);


        iced::widget::column![
            self.summary_row("Daily Summary", &summaries.summary),
            self.summary_row("Daily Payments Breakdown", &summaries.payments),
            self.summary_row("Daily Admission Breakdown", &summaries.admissions),
            self.summary_row("Daily Membership Sales Breakdown", &summaries.memberships),
        ].spacing(RULE_HEIGHT).into()
    }

    pub fn view(&self) -> Element<Message> {
        scrollable(iced::widget::column![
            self.sale_screen.view().map(Message::SaleMessage),
            self.summary(),
            button("Export Daily Report").on_press(RenderDailyReport)
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