use clap::{Parser, Subcommand};
use log::{info, error};

mod queue;
mod logging;

use queue::{Message, MessageQueue};
use logging::setup_logger;

#[derive(Parser)]
#[command(name = "ntfy-send")]
#[command(version = "1.0")]
#[command(about = "Sends messages to ntfy server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(short, long)]
    server: Option<String>,
    
    #[arg(short, long)]
    topic: Option<String>,
    
    #[arg(short, long)]
    message: Option<String>,
    
    #[arg(short, long)]
    username: Option<String>,
    
    #[arg(short, long)]
    password: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Clear the message queue
    ClearQueue,
}

fn main() {
    setup_logger().expect("Failed to setup logger");
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::ClearQueue) => {
            info!("Clearing message queue");
            clear_queue();
        }
        None => {
            if cli.server.is_none() && cli.topic.is_none() && cli.message.is_none() {
                print_help();
                return;
            }
            
            if let (Some(server), Some(topic), Some(message)) = (cli.server, cli.topic, cli.message) {
                send_message(&server, &topic, &message, cli.username, cli.password);
            } else {
                error!("Missing required parameters: server, topic, or message");
                print_help();
            }
        }
    }
}

fn print_help() {
    println!("ntfy-send v1.0");
    println!("Usage:");
    println!("  Send message: ntfy-send -s SERVER -t TOPIC -m MESSAGE [-u USERNAME -p PASSWORD]");
    println!("  Clear queue: ntfy-send clear-queue");
}

fn send_message(server: &str, topic: &str, message: &str, username: Option<String>, password: Option<String>) {
    info!("Attempting to send message to {} on topic {}", server, topic);
    
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");
    
    let mut request = client.post(&format!("{}/{}", server, topic))
        .body(message.to_string());
    
    // Handle credentials without moving them
    if let (Some(u), Some(p)) = (&username, &password) {
        request = request.basic_auth(u, Some(p));
    }
    
    match request.send() {
        Ok(response) => {
            if response.status().is_success() {
                info!("Message sent successfully");
            } else {
                error!("Failed to send message: {}", response.status());
                queue_message(server, topic, message, username, password);
            }
        }
        Err(e) => {
            error!("Failed to send message: {}", e);
            queue_message(server, topic, message, username, password);
        }
    }
}
fn queue_message(server: &str, topic: &str, message: &str, username: Option<String>, password: Option<String>) {
    info!("Queueing message for later delivery");
    
    let mut queue = MessageQueue::load();
    queue.add(Message {
        server: server.to_string(),
        topic: topic.to_string(),
        message: message.to_string(),
        username,
        password,
    });
    queue.save();
}

fn clear_queue() {
    let mut queue = MessageQueue::load();
    let messages = queue.take_all();
    
    for msg in messages {
        send_message(&msg.server, &msg.topic, &msg.message, msg.username, msg.password);
    }
}
