use std::{hash::Hash, io::Read, path::PathBuf};

use iced::{subscription, Subscription, futures::channel::mpsc::UnboundedSender};

pub struct Extraction {
    state: State,
    path: PathBuf,
}

impl Extraction {
    pub fn extract(&mut self) {
        match &self.state {
            State::Ready(path) => {}
            State::Extracting { total, completed } => {}
            State::Finished => {}
        }
    }
}

pub fn extract<I: 'static + Copy + Send + Sync + Hash + Read>(
    id: I,
    file_path: PathBuf,
) -> Subscription<(I, Progress)> {
    subscription::unfold(id, State::Ready(file_path), move |state| {
        extract_file_impl(id, state)
    })
}

async fn extract_file_impl<I: Copy>(file: I, state: State) -> ((I, Progress), State) {
    match state {
        State::Ready(_path) => (
            (file, Progress::Started),
            State::Extracting {
                total: 0,
                completed: 0,
            },
        ),
        State::Extracting { total, completed } => {
            let progress = completed as f32 / total as f32;
            (
                (file, Progress::Advanced(progress)),
                State::Extracting { total, completed },
            )
        }
        State::Finished => iced::futures::future::pending().await,
    }
}

fn extraction_fn_thread(sender: UnboundedSender<(u32, u32)>, pdf_file_path: PathBuf) {
    
}

#[derive(Clone, Copy, Debug)]
pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    Errored,
}

pub enum State {
    Ready(PathBuf),
    Finished,
    Extracting { total: u32, completed: u32 },
}
