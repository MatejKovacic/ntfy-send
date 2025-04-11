use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;

const QUEUE_DIR: &str = "/var/lib/ntfy-send";
const QUEUE_FILE: &str = "unsent-messages-queue.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub server: String,
    pub topic: String,
    pub message: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub struct MessageQueue {
    messages: Vec<Message>,
}

impl MessageQueue {
    pub fn load() -> Self {
        let path = Path::new(QUEUE_DIR).join(QUEUE_FILE);
        
        if !Path::new(QUEUE_DIR).exists() {
            fs::create_dir_all(QUEUE_DIR).expect("Failed to create queue directory");
        }
        
        if path.exists() {
            let data = fs::read_to_string(&path).expect("Failed to read queue file");
            let messages = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
            MessageQueue { messages }
        } else {
            MessageQueue { messages: Vec::new() }
        }
    }
    
    pub fn save(&self) {
        let path = Path::new(QUEUE_DIR).join(QUEUE_FILE);
        let data = serde_json::to_string(&self.messages).expect("Failed to serialize queue");
        fs::write(path, data).expect("Failed to write queue file");
    }
    
    pub fn add(&mut self, message: Message) {
        self.messages.push(message);
    }
    
    pub fn take_all(&mut self) -> Vec<Message> {
        let messages = std::mem::take(&mut self.messages);
        self.save();
        messages
    }
}
