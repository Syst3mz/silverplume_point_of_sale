use crate::app::App;
mod app;
mod as_description;
mod decimal_input;
mod sale_screen;
mod database;
mod model;
mod view;
mod to_model;

pub const HEADER_SIZE: u16 = 32;
pub const TEXT_SIZE: u16 = 18;
pub const RULE_HEIGHT: u16 = 8;

fn main() -> iced::Result {
    //todo: Keep track of hours for admissions entries.

    env_logger::init();
    iced::application(
        "Museum Point Of Sale",
        App::update,
        App::view
    )
    .font(iced_fonts::REQUIRED_FONT_BYTES)
    .run()
}
