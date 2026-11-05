# File structure and responsibilities

## Root
- `Cargo.toml`        Rust manifest, dependencies (crossterm, serde_json, reqwest)
- `Cargo.lock`        Locked dependency tree
- `config.ini`        Default/example configuration (`[ai]`, `[app]`, `[history]`)
- `package.json`      Node manifest, references `whyedcode.exe`
- `whyedcode.cmd`     Windows launcher script
- `whyedcode.exe`     Compiled release binary (copied from `target/release/`)
- `LICENSE`, `README.md`

## `src/main.rs`
- Entry point. Declares `mod app;` and calls `app::run()`.

## `src/app/mod.rs`
- Module declarations: `api`, `commands`, `config`, `history`, `input`, `render`, `ui`, `window`.
- `run()`: creates `Config`, `TerminalWindow`, starts event loop.

## `src/app/api.rs`
- `ApiClient`: blocking `reqwest` client.
- `from_config()`: builds client from `Config` (api_key, api_url).
- `chat()`: POSTs JSON `{model:"llama", messages, stream:false}` to `/chat/completions`.
  Parses `choices[0].message.content` or `error.message` from response.
  Returns raw text if JSON parsing fails.

## `src/app/config.rs`
- `Config`: parses/writes `~/.whyed/config.ini` (INI format).
- Provides: `app_name()`, `model()`, `version()`, `accent()`, `ai_key()`, `ai_url()`,
  `history_enabled()`, `history_max_items()`, `history_save_path()`.
- Defaults: `api=token-abc123`, `api_url=http://localhost:8080/v1`,
  `accent=orange`, `version=v2.0.12-dev`.

## `src/app/window.rs`
- `TerminalWindow`: main application state (config, history, api, messages, running).
- `run()`: main REPL loop — read line, handle command or send to AI, print reply.
- Error handling: connection-refused detected via "connection refused"/"tcp connect
  error"/"10061" substrings; prints friendly message.

## `src/app/commands.rs`
- Slash command handlers on `TerminalWindow`:
  `/exit`, `/quit`, `/clear`, `/status`, `/history`, `/ai`, `/help`, `/accent`,
  `/accent <name>`.
- `print_status()`: shows app, model, version, accent, history count, config path,
  cwd, masked AI key and URL.
- `mask_key()`: hides API key, shows first/last 4 chars.

## `src/app/input.rs`
- `read_line()`: raw-mode line editor using crossterm.
- Supports: Enter, Ctrl+C, Ctrl+D, Backspace, Delete, arrows, Home, End,
  Up/Down history navigation, regular chars.
- Prompt uses `VISIBLE_SPACE` (U+2003 EM SPACE) instead of ASCII space.

## `src/app/ui.rs`
- `VISIBLE_SPACE: &str = "\u{2003}"` — EM SPACE, wider than normal space.
- `visible_width()`: computes display width (combining chars skipped, wide CJK = 2).
- `strip_ansi()`: removes ANSI escape sequences.
- `pad_line()`: pads content to width using `VISIBLE_SPACE`.
- `shorten_path()`: shortens home directory path to `~/`
- `term_width()`: terminal width, default 80.

## `src/app/render.rs`
- `render_box()`: draws bordered box around content lines using box-drawing chars.
- `build_content()`: welcome banner lines.
- `clear()`, `set_title()`: terminal control.

## `src/app/history.rs`
- `History`: appends user input to `~/.whyed/history.json`, loads on start,
  caps at `max_items`, avoids duplicate consecutive entries.

## `target/`
- Build artifacts. Release binary at `target/release/whyedcode.exe`.
