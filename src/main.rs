mod extraction;

use std::ops::RangeInclusive;
use std::path::PathBuf;

use iced::widget::{button, progress_bar, Column};
use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length, Settings, Application, executor, Theme, Command, Subscription};
use native_dialog::FileDialog;
struct Vortex {
    pdf_path: PathBuf,
    state: AppState,
}

enum AppState {
    Init,
    Processing,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FileSelect,
    ExtractionProgress(f32)
}

impl Application for Vortex {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags:()) -> (Vortex, Command<Message>) {
        (Vortex {
            pdf_path: PathBuf::new(),
            state: AppState::Init,
        },
        Command::none())
    }
    fn title(&self) -> String {
        "Vortex UI".to_string()
    }
    fn view(&self) -> Element<Message> {
        match self.state {
            AppState::Init => {
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

            AppState::Processing => {
                let range_progress = RangeInclusive::new(0.0, 100.0);
                let progress = progress_bar(range_progress, 50.0);

                container(progress)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_y()
                    .center_x()
                    .into()
            }
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FileSelect => {
                let path = FileDialog::new()
                    .set_location("~")
                    .add_filter("PDF File", &["pdf"])
                    .show_open_single_file()
                    .unwrap();
                self.pdf_path = path.unwrap();
                self.state = AppState::Processing;
            }
            Message::ExtractionProgress(progress) => {

            }
        };

        Command::none()
    }
}

fn main() {
    Vortex::run(Settings::default()).unwrap();
}
