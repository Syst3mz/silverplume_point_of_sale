use iced::Element;
use iced::widget::{text, text_input};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DecimalInput {
    field: String,
    label: String,
    value: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Change(String),
}

impl DecimalInput {
    pub fn new(label: impl AsRef<str>, default_value: f32) -> Self {
        Self {
            label: label.as_ref().to_string(),
            field: default_value.to_string(),
            value:default_value,
        }
    }
    pub fn value(&self) -> f32 {
        self.value
    }
    fn handle_change(&mut self, text: String) {
        for char in text.chars() {
            if !char.is_numeric() && char != '.' {
                return;
            }
        }

        self.field = text;
        
        if let Ok(number) = self.field.parse::<f32>() {
            self.value = number;
            
        }
    }
    
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Change(text) => self.handle_change(text),
        }
    }
    pub fn view(&self) -> Element<Message> {
        iced::widget::row![
            text(&self.label),
            text_input(&self.value.to_string(), &self.field).on_input(Message::Change),
        ].into()
    }
}

impl Default for DecimalInput {
    fn default() -> Self {
        Self {
            field: String::new(),
            label: String::new(),
            value: 0.0,
        }
    }
}