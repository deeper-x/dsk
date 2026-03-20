# deepseek-chat

An interactive CLI chat app for DeepSeek's API, written in Rust.

## Setup

1. Get your API key from https://platform.deepseek.com
2. Export it as an environment variable:

```bash
export DEEPSEEK_API_KEY=your_key_here
```

## Build & Run

```bash
cargo build --release
./target/release/deepseek-chat
```

Or run directly:

```bash
cargo run
```

## Usage

```
╔══════════════════════════════════════╗
║  dsk - DeepSeek cli                  ║
║  Type 'exit' or 'quit' to leave      ║
║  Type 'clear' to reset the history   ║
╚══════════════════════════════════════╝

Human> Who are you?

Machine> I'm DeepSeek, an AI assistant...

Human> _
```

### Commands

| Command       | Effect                          |
|---------------|---------------------------------|
| `exit`/`quit` | Exit the app                    |
| `clear`       | Reset conversation history      |
| Ctrl-D        | Exit (EOF)                      |

## Notes

- The full conversation history is sent on every request, giving the model multi-turn memory.
- Using `clear` wipes that history, starting a fresh context.
- The model used is `deepseek-chat` (DeepSeek-V3). Change it in `main.rs` if needed.
