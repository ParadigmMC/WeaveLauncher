use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::mpsc::{channel, Receiver, Sender},
};

pub struct TaskMessage {
    pub message: String,
    pub progress: Option<u8>,
}

pub struct Task {
    pub name: String,
    pub id: String,

    pub recv: Receiver<TaskMessage>,
}

impl Task {
    pub fn create() -> Self {
        let (send, recv) = channel();

    }

    pub fn execute(&self) {

    }

    pub fn is_complete(&self) -> bool {
        true
    }
}
