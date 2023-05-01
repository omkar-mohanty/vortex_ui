use std::path::PathBuf;

use iced::widget::{button, Column};
use iced::widget::{column, container,text};
use iced::{Alignment, Element, Length,Sandbox, Settings};
use native_dialog::FileDialog;
struct Counter {
    pdf_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FileSelect,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter { pdf_path: PathBuf::new() }
    }
    fn title(&self) -> String {
        "Vortex UI".to_string()
    }
    fn view(&self) -> Element<Message> {
        let welcome_text = text("Welcome to Vortex UI!");

        let select_files_button = button("Select file")
            .on_press(Message::FileSelect)
            .padding(10);

        let column: Column<_> = column![welcome_text, select_files_button]
            .align_items(Alignment::Center)
            .spacing(10);

        container(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::FileSelect => {
                let path = FileDialog::new()
                    .set_location("~")
                    .add_filter("PDF File", &["pdf"])
                    .show_open_single_file()
                    .unwrap();
                self.pdf_path = path.unwrap();
            }
        }
    }
}

fn main() {
    Counter::run(Settings::default()).unwrap();
}
