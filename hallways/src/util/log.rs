use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Debug,
    Info,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub level: Level,
    pub message: String,
}

pub struct Listener {
    receiver: Receiver<Message>,
}

static LISTENERS: Mutex<Vec<Sender<Message>>> = Mutex::new(Vec::new());

impl Listener {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        LISTENERS.lock().unwrap().push(sender);
        return Self { receiver };
    }

    pub fn get_message(&self) -> Option<Message> {
        return self.receiver.try_recv().ok();
    }
}

pub fn log(level: Level, message: String) {
    let message = Message { level, message };

    for listener in LISTENERS.lock().unwrap().iter() {
        let _ = listener.send(message.clone());
    }
}
