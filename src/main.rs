use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "piper")]
#[command(about = "A local-only chat room for ants", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a message to a chat room
    Say {
        /// The message to send
        #[arg(short, long)]
        message: String,

        /// The room name
        #[arg(short, long)]
        room: String,

        /// Your user ID
        #[arg(short, long)]
        user_id: String,
    },
    /// Read the last 10 messages from a chat room
    Read {
        /// The room name
        #[arg(short, long)]
        room: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Message {
    room: String,
    user_id: String,
    message: String,
    timestamp: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Say {
            message,
            room,
            user_id,
        } => {
            say_message(&message, &room, &user_id)?;
        }
        Commands::Read { room } => {
            read_messages(&room)?;
        }
    }

    Ok(())
}

fn say_message(message: &str, room: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME")?;
    let piper_dir = PathBuf::from(home_dir).join(".piper");

    create_dir_all(&piper_dir)?;

    let room_file = piper_dir.join(format!("{}.jsonl", room));

    let msg = Message {
        room: room.to_string(),
        user_id: user_id.to_string(),
        message: message.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let json_line = serde_json::to_string(&msg)?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(room_file)?;

    writeln!(file, "{}", json_line)?;

    println!("Message sent to room '{}'", room);

    Ok(())
}

fn read_messages(room: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME")?;
    let piper_dir = PathBuf::from(home_dir).join(".piper");
    let room_file = piper_dir.join(format!("{}.jsonl", room));

    if !room_file.exists() {
        println!("Room '{}' has no messages yet", room);
        return Ok(());
    }

    let file = std::fs::File::open(room_file)?;
    let reader = BufReader::new(file);

    let mut messages: Vec<Message> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Ok(msg) = serde_json::from_str::<Message>(&line) {
            messages.push(msg);
        }
    }

    if messages.is_empty() {
        println!("Room '{}' has no messages yet", room);
        return Ok(());
    }

    let last_10 = messages.iter().rev().take(10).rev();

    println!("Last messages in room '{}':\n", room);
    for msg in last_10 {
        let timestamp = chrono::DateTime::parse_from_rfc3339(&msg.timestamp)?;
        let local_time = timestamp.with_timezone(&chrono::Local);
        println!(
            "[{}] {}: {}",
            local_time.format("%Y-%m-%d %H:%M:%S"),
            msg.user_id,
            msg.message
        );
    }

    Ok(())
}
