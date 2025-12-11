pub struct Logger {
    messages: Vec<String>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn log_step(&mut self, msg: &str) {
        self.messages.push(msg.to_string());
    }

    pub fn messages(&self) -> &[String] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}