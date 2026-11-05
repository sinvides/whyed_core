# Whyed Code — Project Instructions for AI Assistants

## Project Overview
Whyed Code is a terminal-based AI pair programmer written in Rust. It runs in the console, reads user input, sends it to a local AI server (OpenAI-compatible `/v1/chat/completions`), and prints assistant replies. It supports slash commands, persistent history, customizable terminal accents, and full file management.

## Architecture

### Core Modules (`src/app/`)
- `core/state.rs` — Global `AppState` shared across subsystems
- `core/permissions.rs` — Permission system (Ask / AcceptEdits / Plan modes)
- `core/logger.rs` — Structured, eye-friendly logging with levels
- `ui/colors.rs` — Dark, muted ANSI 256 color palette
- `ui/theme.rs` — Theme context
- `ui/layout.rs` — Visible-width padding, terminal size
- `ui/renderer.rs` — Box rendering, screen clearing
- `input/editor.rs` — Raw-mode line editor with visible-space prompt
- `input/parser.rs` — Slash command + `@file` reference parsing
- `files/reader.rs` — FileRead tool (read files/directories)
- `files/writer.rs` — FileWrite tool (create/overwrite files)
- `files/editor.rs` — FileEdit tool (string replacement in files)
- `files/search.rs` — Glob/Grep file search
- `ai/client.rs` — HTTP client for local AI server
- `ai/chat.rs` — Conversation history management
- `commands/mod.rs` — Command registry and dispatch
- `commands/help.rs` — `/help` command
- `commands/status.rs` — `/status` command
- `commands/file_read.rs` — `/read` command
- `commands/file_write.rs` — `/write` command
- `commands/file_edit.rs` — `/edit` command
- `commands/file_search.rs` — `/search` command

## Key Conventions

### Colors
Use the dark theme from `ui/colors::Colors::dark()`. Colors are muted ANSI 256 codes — avoid neon/overexposed tones.
- `accent` — muted blue (primary)
- `secondary` — muted purple
- `success` — soft green
- `warning` — soft amber
- `error` — soft red
- `muted` — gray
- `text` — muted off-white

### Visible Spaces
Use `VISIBLE_SPACE` (`\u{2003}` EM SPACE) instead of ASCII space for indentation — it is visibly wider.

### Permission Modes
- `Ask` — prompt user before every file mutation (default)
- `AcceptEdits` — apply edits automatically
- `Plan` — only show what would change, don't apply

### @file References
Users can reference files in messages using `@filename` syntax. The system reads the file and includes it in the AI context.

### Slash Commands
All commands live in `src/app/commands/`. Each implements the `CommandHandler` trait with `name()`, `description()`, and `handle()`.

## Building
```bash
cargo build --release
copy target/release/whyedcode.exe whyedcode.exe
```

## Configuration
Config: `~/.whyed/config.ini`
- `[ai] api_url` — local AI server base URL (default: `http://localhost:8080/v1`)
- `[ai] api` — Bearer token
- `[app] accent` — terminal accent color
- `[history] enabled` — save history to disk

## Testing
```bash
cargo test
```

## Style Guidelines
- All user-facing strings in Russian (error messages) / English (commands)
- Use `logger::info/warn/error/success` for structured output
- Always create parent directories before writing files
- Handle `io::Result` properly, don't unwrap in production code
