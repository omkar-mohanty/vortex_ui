use crate::{Message, Result};
use std::{
    hash::Hash,
    path::PathBuf,
    thread::{self, JoinHandle},
};

use iced::{subscription, Subscription};
use vortex::{extractor::extract_images, RawImage};

pub struct Extraction {
    id: i32,
    state: State,
    pub progress: i32
}

impl Extraction {
    pub fn new(pdf_file_path: PathBuf) -> Self {
        Self {
            state: State::Ready(pdf_file_path),
            id: 0,
            progress:0
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            State::Ready(path) => extract(self.id, path.to_path_buf())
                .map(|progress| Message::ExtractionProgress(progress)),
            _ => Subscription::none(),
        }
    }
}

pub fn extract<I: 'static + Copy + Send + Sync + Hash>(
    id: I,
    file_path: PathBuf,
) -> Subscription<Progress> {
    subscription::unfold(id, State::Ready(file_path), move |state| {
        extract_file_impl(id, state)
    })
}

async fn extract_file_impl<I: Copy>(_id: I, state: State) -> (Progress, State) {
    match state {
        State::Ready(path) => {
            let handle = thread::spawn(move || {
                match extraction_fn_thread(path) {
                    Ok(img) => img,
                    Err(_) => vec![]
                }
            });

            (Progress::Started, State::Extracting(handle))
        }
        State::Extracting(handle) => {
            handle.join().unwrap();

            (Progress::Finished, State::Finished)
        }
        State::Finished => iced::futures::future::pending().await,
    }
}

fn extraction_fn_thread(pdf_file_path: PathBuf) -> Result<Vec<RawImage>> {
    let images = extract_images(vortex::extractor::Method::File(pdf_file_path))?;
    Ok(images)
}

#[derive(Clone, Copy, Debug)]
pub enum Progress {
    Started,
    Advanced,
    Finished,
    Errored,
}

pub enum State {
    Ready(PathBuf),
    Finished,
    Extracting(JoinHandle<Vec<RawImage>>),
}
