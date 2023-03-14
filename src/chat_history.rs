use crate::ChatCompletionMessage;

pub struct ChatHistory {
    messages: Vec<ChatCompletionMessage>,
}

impl ChatHistory {
    pub fn new() -> Self {
        ChatHistory {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: ChatCompletionMessage) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<ChatCompletionMessage> {
        &self.messages
    }
}
