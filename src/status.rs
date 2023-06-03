use std::time::{Duration, Instant};

pub struct StatusMessage {
    message: Option<String>,
    set_time: Option<Instant>,
}

impl StatusMessage {
    pub fn new(initial_message: &str) -> Self {
        Self {
            message: Some(initial_message.into()),
            set_time: Some(Instant::now()),
        }
    }

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
        self.set_time = Some(Instant::now());
    }

    pub fn message(&mut self) -> Option<&String> {
        self.set_time
            .and_then(|time| {
                if time.elapsed() > Duration::from_secs(5) {
                    self.message = None;
                    self.set_time = None;
                    None
                } else {
                    Some(self.message.as_ref().unwrap())
                }
            })
    }
}


