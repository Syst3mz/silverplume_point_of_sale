use iced::Element;
use iced::widget::text_input;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DecimalInput {
    field: String,
    placeholder: String,
    value: f32,
    error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Change(String),
}

impl DecimalInput {
    pub fn new(placeholder: impl AsRef<str>) -> Self {
        Self {
            field: String::new(),
            placeholder: placeholder.as_ref().to_string(),
            value: 0.0,
            error: None,
        }
    }
    pub fn value(&self) -> f32 {
        self.value
    }
    fn handle_change(&mut self, text: String) {
        self.field = text;
        match self.field.parse::<f32>() {
            Ok(f) => {
                self.error = None;
                self.value = f;
            }
            Err(e) => {
                self.error = Some(e.to_string());
            }
        }
    }
    
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Change(text) => self.handle_change(text),
        }
    }
    pub fn view(&self) -> Element<Message> {
        let mut column = iced::widget::column![
            text_input(&self.placeholder, &self.field).on_input(Message::Change),
        ];
        if self.error.is_some() {
            column = column.push(iced::widget::Text::new(self.error.as_ref().unwrap()));
        }
        column.into()
    }
    
    pub fn set_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }
}

impl Default for DecimalInput {
    fn default() -> Self {
        Self {
            field: String::new(),
            placeholder: String::new(),
            value: 0.0,
            error: None,
        }
    }
}