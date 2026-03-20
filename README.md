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
$ cargo run 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/dsk`
╔════════════════════════════════════════════╗
║  dsk - DeepSeek cli                        ║
║  Type 'exit' or 'quit' to leave            ║
║  Type 'clear' to reset the history         ║
╚════════════════════════════════════════════╝

Human> explain what dyn means in rust

Machine> In Rust, dyn is a keyword used to create trait objects, which enable dynamic dispatch (runtime polymorphism). Here's a comprehensive explanation:

What dyn Means

dyn is short for "dynamic" and indicates that method calls on this trait will be resolved at runtime rather than compile time.

Basic Syntax:

// Trait definition                         
trait Animal {                              
    fn speak(&self);                        
}                                           


// Using dyn to create a trait object       
let animal: &dyn Animal = &Cat;             
// or                                       
let animal: Box<dyn Animal> = Box::new(Dog);
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

## TODO 
The model used is `deepseek-chat` (DeepSeek-V3). Change it in settings.
