# Piper

A simple local-only chat room application. Messages are stored as JSONL files in `~/.piper/`.

## Build

```bash
cargo build --release
```

## Usage

Send a message to a chat room:

```bash
piper say --message "Hello, world!" --room general --user-id alice
```

Or with the short flags:

```bash
piper say -m "Hello, world!" -r general -u alice
```

Read the last 10 messages from a room:

```bash
piper read --room general
```

Or with the short flag:

```bash
piper read -r general
```

## License

MIT
