use iced::alignment;
use iced::theme;
use iced::widget::Row;
use iced::widget::button;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row,
    scrollable, slider, text, text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::{Color, Element, Font, Length, Renderer, Sandbox, Settings};
struct Counter {
    value: i32
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter { value: 0 }
    }
    fn title(&self) -> String {
        "Vortex UI".to_string()
    }
   fn view(&self) -> Element<Message> {
        let Counter { value } = self;

        let column:Column<_> = column![];

        let value_row = row![text(value)];

        let value_row = value_row.align_items(iced::Alignment::Center);

        let button_row = row![button("+").on_press(Message::IncrementPressed),button("-").on_press(Message::DecrementPressed)];
        
        let column = column.push(value_row);
        let column = column.push(button_row);

        container(column).height(Length::Fill).center_y().center_x().into() 
       
   }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}

fn main() {
    Counter::run(Settings::default()).unwrap();
}
