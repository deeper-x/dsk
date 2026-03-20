use crate::machine::response::send_message;
use crate::mytypes::chat::Message;
use reqwest::Client;
use std::io::{self, BufRead, Write};
use termimad::MadSkin;

pub fn print_motd() -> () {
    println!("╔════════════════════════════════════════════╗");
    println!("║  dsk - DeepSeek cli                        ║");
    println!("║  Type 'exit' or 'quit' to leave            ║");
    println!("║  Type 'clear' to reset the history         ║");
    println!("╚════════════════════════════════════════════╝");
    println!();
}

pub async fn run() -> () {
    let mut history: Vec<Message> = Vec::new();

    let stdin = io::stdin();

    loop {
        print!("Human> ");
        io::stdout().flush().unwrap();

        // Read a line from stdin
        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => break, // EOF (e.g. Ctrl-D)
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {e}");
                break;
            }
        }

        let input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        match input.to_lowercase().as_str() {
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "clear" => {
                history.clear();
                println!("[Conversation history cleared]\n");
                continue;
            }
            _ => {}
        }

        // Append user message to history
        history.push(Message {
            role: "user".to_string(),
            content: input.clone(),
        });

        let client = Client::new();
        let skin = MadSkin::default();

        let api_key = std::env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| {
            eprintln!("Error: DEEPSEEK_API_KEY environment variable not set.");
            eprintln!("Export it with: export DEEPSEEK_API_KEY=<your_key>");
            std::process::exit(1);
        });

        // Call DeepSeek API
        match send_message(&client, &api_key, &history).await {
            Ok(reply) => {
                print!("\nMachine> ");
                io::stdout().flush().unwrap();
                skin.print_text(&reply);
                println!();

                // Append assistant reply to history for multi-turn context
                history.push(Message {
                    role: "assistant".to_string(),
                    content: reply,
                });
            }
            Err(e) => {
                eprintln!("\n[API Error] {e}\n");
                // Remove the last user message so the history stays consistent
                history.pop();
            }
        }
    }
}
