use crate::app::App;

mod transaction_record;
mod admission;
mod donation;
mod membership;
mod payment_method;
mod gift_shop_sale;
mod app;
mod as_transaction_record;
mod as_description;
mod decimal_input;
mod sale_screen;

pub const HEADER_SIZE: u16 = 32;
pub const TEXT_SIZE: u16 = 16;
pub const RULE_HEIGHT: u16 = 8; 

fn main() -> iced::Result {
    iced::application(
        "Museum Point Of Sale", 
        App::update, 
        App::view
    )
    .font(iced_fonts::REQUIRED_FONT_BYTES)
    .run()
}
