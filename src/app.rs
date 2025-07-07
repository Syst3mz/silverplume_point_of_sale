use std::fmt::Display;
use chrono::{Datelike, Local};
use iced::advanced::Widget;
use iced::alignment::Horizontal;
use iced::{Element, Length};
use iced::widget::{button, container, horizontal_rule, scrollable, text};
use iced_aw::selection_list;
use log::{error, info};
use crate::{HEADER_SIZE, RULE_HEIGHT, TEXT_SIZE};
use crate::app::Message::{CartSelection, CommitToDb, DeleteSelected, RenderDailyReport};
use crate::database::Database;
use crate::database::database_object::CanBuildObjectMapper;
use crate::model::as_transaction_record::AsTransactionRecord;
use crate::model::cart_item::CartItem;
use crate::model::date_time_wrapper::WrapInDateTime;
use crate::model::has_total_cost::HasTotalCost;
use crate::sale_screen::SaleScreen;
use crate::to_model::ToModel;
use crate::view::adapters::ff;
use crate::view::summary_dicts::SummaryDicts;

pub struct App {
    sale_screen: SaleScreen,
    database: Database,
    error: Option<anyhow::Error>,
    cart: Vec<CartItem>,
    stringified_cart: Vec<String>,
    selected_index: Option<usize>,
}

type SaleMessage = crate::sale_screen::Message;
#[derive(Debug, Clone)]
pub enum Message {
    SaleMessage(SaleMessage),
    RenderDailyReport,
    CartSelection(usize),
    DeleteSelected,
    CommitToDb
}

impl App {    
    fn transactionify_and_insert<T>(&mut self, object: T) -> anyhow::Result<()> where
        T: CanBuildObjectMapper+AsTransactionRecord+WrapInDateTime + 'static,
    {
        self.database.insert(object.as_transaction_record().wrapped_in_date_time())?;
        self.database.insert(object.wrapped_in_date_time())
    }

    /*
    self.transactionify_and_insert(self.sale_screen.admission().clone())
self.transactionify_and_insert(self.sale_screen.donation().clone())
self.transactionify_and_insert(self.sale_screen.membership().clone())
self.transactionify_and_insert(self.sale_screen.gift_shop_sale().clone())
    */

    fn add_to_cart<T:Into<CartItem>+Display, S: ToModel<ModelType=T>>(&mut self, item: S) -> anyhow::Result<()> {
        let item = item.to_model()?;
        self.stringified_cart.push(item.to_string());
        self.cart.push(item.into());


        Ok(())
    }

    fn handle_sale_message(&mut self, message: SaleMessage) {
        // For the love of all that is good, do the sale screen update AFTER the transaction is written to the database.
        let err = match message.clone() {
            SaleMessage::AddAdmission => self.add_to_cart(self.sale_screen.admission().clone()),
            SaleMessage::AddDonation => self.add_to_cart(self.sale_screen.donation().clone()),
            SaleMessage::AddMembership => self.add_to_cart(self.sale_screen.membership().clone()),
            SaleMessage::AddGiftShopSale => self.add_to_cart(self.sale_screen.gift_shop_sale().clone()),
            _ => {Ok(())}
        };

        if let Err(err) = err {
            error!("Error inserting item into cart: {}", err);
            self.error = Some(err);
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
                let filename = format!("{}{}{}_report.html", now.year(), now.month(), now.day());
                info!("Exported file to: {}", filename);
                let _ =std::fs::write(
                    filename,
                    self.database.render_to_html()
                );
            },
            Message::CartSelection(c) => self.selected_index = Some(c),
            Message::DeleteSelected => {
                let Some(index) = self.selected_index else {return;};
                self.cart.remove(index);
                self.stringified_cart.remove(index);
                self.selected_index = None;
            },
            Message::CommitToDb => {
                if self.cart.is_empty() {
                    return;
                }

                for item_index in (0..self.cart.len()).rev() {
                    let err = match &self.cart[item_index] {
                        CartItem::Admission(x) => self.transactionify_and_insert(x.clone()),
                        CartItem::Membership(x) => self.transactionify_and_insert(x.clone()),
                        CartItem::Donation(x) => self.transactionify_and_insert(x.clone()),
                        CartItem::GiftShopSale(x) => self.transactionify_and_insert(x.clone()),
                    };

                    if let Err(err) = err {
                        error!("Error inserting item into db: {}", err);
                        self.error = Some(err);
                    } else {
                        self.error = None;
                    }

                    self.cart.remove(item_index);
                    self.stringified_cart.remove(item_index);
                }
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

    fn cost_of_cart(&self) -> String {
        ff("$", self.cart.iter().map(|x| x.total_cost()).sum())
    }

    pub fn view(&self) -> Element<Message> {
        container(scrollable(iced::widget::column![
            iced::widget::row![
                iced::widget::column![
                    text("Cart").size(HEADER_SIZE),
                    selection_list(&self.stringified_cart, |x, _| {CartSelection(x)}).height(400),
                    iced::widget::row![
                        text(format!("Amount Due: {}", self.cost_of_cart())).size(TEXT_SIZE),
                        button("Remove Selected Item").on_press_maybe(self.selected_index.map(|_| DeleteSelected)),
                        button("Save Transaction and Clear Cart").on_press(CommitToDb),
                    ].spacing(RULE_HEIGHT)
                ].padding(RULE_HEIGHT as f32).spacing(RULE_HEIGHT).width(Length::FillPortion(1)),
                container(self.sale_screen.view().map(Message::SaleMessage)).width(Length::FillPortion(2))
            ],
            self.summary(),
            text("Exports").size(HEADER_SIZE).width(Length::Fill).align_x(Horizontal::Center),
            horizontal_rule(RULE_HEIGHT),
            container(button("Export Daily Report").on_press(RenderDailyReport)).width(Length::Fill).align_x(Horizontal::Center)
        ].spacing(2 * RULE_HEIGHT))).padding(RULE_HEIGHT).into()

    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            sale_screen: Default::default(),
            database: Database::new(),
            error: None,
            cart: vec![],
            stringified_cart: vec![],
            selected_index: None,
        }
    }
}