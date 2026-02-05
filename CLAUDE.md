# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Piper is a local-only chat room application written in Rust. It stores messages in JSONL files in the user's home directory (`~/.piper/`). Each chat room gets its own file (`<room-name>.jsonl`).

## Development Commands

### Build
```bash
cargo build              # Debug build
cargo build --release    # Release build
```

### Run
```bash
cargo run -- say --message "Hello" --room general --user-id alice
```

### Testing
```bash
cargo test                    # Run all tests
cargo test <test_name>        # Run specific test
cargo test -- --nocapture     # Show println output from tests
```

### Linting
```bash
cargo clippy              # Run linter
cargo fmt                 # Format code
cargo fmt -- --check      # Check formatting without modifying
```

## Architecture

### CLI Structure
- Built with `clap` using derive macros for argument parsing
- Subcommands:
  - `say`: Send a message to a room (args: `--message`, `--room`, `--user-id`)
  - `read`: Display the last 10 messages from a room (args: `--room`)

### Data Model
- **Message struct**: Contains `room`, `user_id`, `message`, and `timestamp` (RFC3339 format)
- Messages are serialized to JSON using `serde_json`
- Storage format: JSONL (one JSON object per line) in `~/.piper/<room>.jsonl`

### Storage
- All data stored in `~/.piper/` directory (created automatically)
- Each room is a separate `.jsonl` file
- Messages are appended atomically using `OpenOptions::append()`
- Timestamps generated with `chrono::Utc::now().to_rfc3339()`

### Code Organization
- Single file application: `src/main.rs`
- Main function handles CLI parsing and dispatches to command handlers
- `say_message()` function handles message persistence
