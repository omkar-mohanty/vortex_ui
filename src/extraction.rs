use std::{hash::Hash, io::Read,path::PathBuf};

use iced::{Subscription, subscription};

pub struct Extraction {
    state: State
}

pub fn extract<I: 'static + Copy + Send +Sync + Hash + Read>(id: I, file_path: PathBuf) -> Subscription<Progress> {
   subscription::unfold(id, State::Ready(file_path), move |state| {
        extract_file_impl(id, state)
    }) 
}

async fn extract_file_impl<I:Copy>(file:I, state: State) ->((I,Progress), State) {

}

pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    Errored
}

pub enum State {
    Ready(PathBuf),
    Finished,
    Extracting {
        total: u32,
        completed: u32,
    }
}

