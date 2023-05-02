use std::{hash::Hash, io::Read, path::PathBuf};

use iced::{subscription, Subscription};

pub struct Extraction {
    state: State,
    path: PathBuf,
}

impl Extraction {
   pub fn extract(&self) {

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
       State::Ready(_path) => {
            ((file, Progress::Started), State::Extracting { total: 0, completed: 0 })
        },
        State::Extracting { total, completed } => {
            let progress = completed as f32/ total as f32;
            ((file, Progress::Advanced(progress)), State::Extracting { total, completed})
        },
        State::Finished => {
            iced::futures::future::pending().await
        }
    }
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
