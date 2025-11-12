pub struct MessageState {
    message: Option<String>,
}

impl MessageState {
    pub fn new(initial_message: &str) -> Self {
        Self {
            message: Some(initial_message.to_string()),
        }
    }

    pub fn empty() -> Self {
        Self { message: None }
    }

    pub fn get(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn set(&mut self, msg: String) {
        self.message = Some(msg);
    }

    pub fn clear(&mut self) {
        self.message = None;
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }
}