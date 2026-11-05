use serde_json::Value;

/// Manages conversation history and message assembly.
pub struct Chat {
    pub messages: Vec<Value>,
    max_context: usize,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            max_context: 50,
        }
    }

    pub fn add_user(&mut self, content: String) {
        self.messages.push(json!({"role": "user", "content": content}));
        self.trim();
    }

    pub fn add_assistant(&mut self, content: String) {
        self.messages.push(json!({"role": "assistant", "content": content}));
        self.trim();
    }

    pub fn messages(&self) -> &[Value] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    fn trim(&mut self) {
        while self.messages.len() > self.max_context {
            self.messages.remove(0);
        }
    }
}

use serde_json::json;
