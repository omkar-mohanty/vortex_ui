use crate::Result;
use std::{hash::Hash, io::Read, path::PathBuf};

use iced::{subscription, Subscription};
use vortex::extractor::extract_images;

pub fn extract<I: 'static + Copy + Send + Sync + Hash + Read>(
    id: I,
    file_path: PathBuf,
) -> Subscription<State> {
    subscription::unfold(id, State::Ready(file_path), move |state| {
        extract_file_impl(id, state)
    })
}

async fn extract_file_impl<I: Copy>(id: I, state: State) -> ((I, Progress), State) {
    match state {
        State::Ready(path) => {
            let images = extract_images(vortex::extractor::Method::File(path)).unwrap();
            ((id, Progress::Started), State::Extracting)
        }
        State::Extracting => {
            ((id, Progress::Advanced(progress)), State::Extracting)
        }
        State::Finished => iced::futures::future::pending().await,
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
    Extracting,
}
