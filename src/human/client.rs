use crate::data_models::ai::{Message, Role};
use crate::machine::engine::send_message;
use crate::settings;
use io::Error;
use log::error;
use reqwest::Client;

use std::io::{self, BufRead, Write};
use termimad::MadSkin;

enum PS1 {
    Red,
    Blue,
}

impl PS1 {
    // see https://en.wikipedia.org/wiki/ANSI_escape_code for
    // ansi color codes list (red: 31, blue: 34, ...)
    fn code(&self) -> &'static str {
        match self {
            PS1::Blue => "34",
            PS1::Red => "31",
        }
    }

    fn print(&self, role: Role) {
        let label = match role {
            Role::User => "You ",
            Role::Assistant => "AI ",
        };

        print!("\x1b[{}m{}>\x1b[0m ", self.code(), label);
    }
}

fn print_motd() -> () {
    println!("╔════════════════════════════════════════════╗");
    println!("║  dsk - DeepSeek cli                        ║");
    println!("║  Type 'exit' or 'quit' to leave            ║");
    println!("║  Type 'clear' to reset the history         ║");
    println!("╚════════════════════════════════════════════╝");
    println!();
}

pub fn get_api_key_env() -> String {
    let api_key = std::env::var(settings::api::DEEPSEEK_API_KEY);

    match api_key {
        Ok(key) => key,
        Err(e) => {
            eprintln!("error running api key env: {e}");
            error!("error running api key env: {e}");

            std::process::exit(1);
        }
    }
}

pub async fn run(api_key: String) -> () {
    print_motd();

    let mut history: Vec<Message> = Vec::new();

    let stdin: io::Stdin = io::stdin();

    loop {
        PS1::Blue.print(Role::User);

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
                PS1::Red.print(Role::Assistant);

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
