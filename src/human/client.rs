use crate::data_models::ai::{Message, Role};
use crate::data_models::errors::AIError;
use crate::machine::engine::send_message;
use crate::settings;
use io::Error;
use reqwest::Client;

use std::io::{self, BufRead, Write};
use termimad::MadSkin;

fn print_motd() -> () {
    println!("╔════════════════════════════════════════════╗");
    println!("║  dsk - DeepSeek cli                        ║");
    println!("║  Type 'exit' or 'quit' to leave            ║");
    println!("║  Type 'clear' to reset the history         ║");
    println!("╚════════════════════════════════════════════╝");
    println!();
}

fn print_ps1(role: Role) -> () {
    match role {
        Role::User => {
            print!("You> ")
        }
        Role::Assistant => {
            print!("AI> ")
        }
    }
}

pub fn get_api_key_env() -> Result<String, AIError> {
    let api_key = std::env::var(settings::api::DEEPSEEK_API_KEY);

    match api_key {
        Ok(key) => Ok(key),
        Err(e) => Err(AIError::Env(e)),
    }
}

pub async fn run(api_key: String) -> () {
    print_motd();

    let mut history: Vec<Message> = Vec::new();

    let stdin: io::Stdin = io::stdin();

    loop {
        print_ps1(Role::User);

        let out: Result<(), Error> = io::stdout().flush();

        match out {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error handling output: {e}");
                break;
            }
        }

        // Read a line from stdin
        let mut input: String = String::new();
        let line: Result<usize, Error> = stdin.lock().read_line(&mut input);

        match line {
            Ok(0) => break, // EOF (e.g. Ctrl-D)
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {e}");
                break;
            }
        }

        let input_str: String = input.trim().to_string();

        if input_str.is_empty() {
            continue;
        }

        match input_str.to_lowercase().as_str() {
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
            role: Role::User.to_string(),
            content: input_str.clone(),
        });

        let client: Client = Client::new();
        let skin: MadSkin = MadSkin::default();

        // Call DeepSeek API
        match send_message(&client, &api_key, &history).await {
            Ok(reply) => {
                print_ps1(Role::Assistant);

                let out: Result<(), Error> = io::stdout().flush();

                match out {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error handling output: {e}");
                        break;
                    }
                }

                skin.print_text(&reply);
                println!();

                // Append assistant reply to history for multi-turn context
                history.push(Message {
                    role: Role::Assistant.to_string(),
                    content: reply,
                });
            }
            Err(e) => {
                eprintln!("\n[API Error] {e}\n");
                // Remove the last user message so the history stays consistent
                history.pop();
                break;
            }
        }
    }
}
