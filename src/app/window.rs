use crate::app::ai::client::ApiClient;
use crate::app::ai::chat::Chat;
use crate::app::commands::dispatch;
use crate::app::config::Config;
use crate::app::core::PermissionMode;
use crate::app::core::state::AppState;
use crate::app::core::logger;
use crate::app::history::History;
use crate::app::input::editor::LineEditor;
use crate::app::input::parser::parse;
use crate::app::ui::colors::VISIBLE_SPACE;
use crate::app::ui::renderer::Renderer;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::{self, Write};
use std::path::Path;

pub struct TerminalWindow {
    pub state: AppState,
    pub renderer: Renderer,
}

impl TerminalWindow {
    pub fn new(title: &str) -> Self {
        let config = Config::new();
        let history_file = config.history_save_path();
        let max_items = config.history_max_items();
        let enabled = config.history_enabled();
        let history = History::new(&history_file, max_items, enabled);
        let api = ApiClient::from_config(&config);
        let chat = Chat::new();

        let mut state = AppState::new(config, history, api);
        state.messages = chat.messages;

        // Auto-load WHYED.md if present (like WHYED Code)
        Self::load_whyed_md(&mut state);

        // Set terminal title
        let _ = crate::app::ui::renderer::Renderer::set_title(title);

        Self {
            state,
            renderer: Renderer::new(),
        }
    }

    fn load_whyed_md(state: &mut AppState) {
        let path = Path::new("WHYED.md");
        if path.exists() {
            match crate::app::files::reader::FileReader::read_file(path) {
                Ok(content) => {
                    logger::info("WHYED.md", &format!("Loaded project instructions ({} bytes)", content.len()));
                    state.messages.push(serde_json::json!({
                        "role": "system",
                        "content": format!("Project instructions from WHYED.md:\n{}", content)
                    }));
                }
                Err(e) => {
                    logger::warn("WHYED.md", &format!("Cannot read: {}", e));
                }
            }
        }
    }

    pub fn run(&mut self) {
        let _c = &self.renderer.theme.colors;
        let mut stdout = io::stdout();

        // Clear and render welcome
        let _ = Renderer::clear_screen(&mut stdout);
        self.render_welcome(&mut stdout);

        // Main REPL loop
        loop {
            if !self.state.running {
                break;
            }

            // Handle pending permission actions first (Ask mode)
            if self.state.permission_mode == PermissionMode::Ask && self.state.pending_count() > 0 {
                if !self.handle_pending_permission(&mut stdout) {
                    break;
                }
                continue;
            }

            let history_snapshot: Vec<String> = self.state.history.items.clone();

            // Build prompt with visible spaces
            let prompt = self.build_prompt();
            let mut editor = LineEditor::new(&prompt, &history_snapshot);

            let input = match editor.read(&mut stdout) {
                Ok(crate::app::input::editor::LineResult::Submitted(s)) => s,
                Ok(crate::app::input::editor::LineResult::Cancelled) | Err(_) => {
                    print!("\r\n");
                    self.state.running = false;
                    break;
                }
            };

            if input.trim().is_empty() {
                continue;
            }

            // Save to history
            let _ = self.state.history.add(input.trim());

            // Parse and dispatch
            let parsed = parse(&input);
            let should_continue = dispatch(&mut self.state, parsed);
            if !should_continue {
                break;
            }
        }
    }

    fn handle_pending_permission(&mut self, stdout: &mut io::Stdout) -> bool {
        let c = &self.renderer.theme.colors;
        let last_idx = self.state.pending_count() - 1;
        let action = self.state.pending_actions.last().unwrap().clone();

        println!("\r\n");
        match &action {
            crate::app::core::state::PendingAction::Write { path, content } => {
                println!("  {}{}FileWrite{} {}", c.warning, c.accent, c.reset, path.display());
                println!("     {}{}{}", c.text, content, c.reset);
            }
            crate::app::core::state::PendingAction::Edit { path, old, new } => {
                println!("  {}{}FileEdit{} {}", c.warning, c.accent, c.reset, path.display());
                println!("     {}Old:{} {}", c.muted, c.text, old);
                println!("     {}New:{} {}", c.muted, c.text, new);
            }
        }
        print!("     {}(a)llow  (n)one  (p)lan mode{} > ", c.accent, c.reset);
        stdout.flush().ok();

        // Read a single character response
        let response = self.read_single_char(stdout);
        println!("\r\n");

        match response.as_deref() {
            Some("a") | Some("A") => {
                let confirmed = self.state.confirm_pending(last_idx).is_some();
                if confirmed {
                    self.execute_action(&action);
                }
                true
            }
            Some("n") | Some("N") => {
                self.state.reject_pending(last_idx);
                println!("  {}⚠ Rejected.{}", c.warning, c.reset);
                true
            }
            Some("p") | Some("P") => {
                self.state.permission_mode = PermissionMode::Plan;
                self.state.reject_pending(last_idx);
                logger::info("Permission", "Switched to Plan mode");
                true
            }
            _ => {
                println!("  {}⚠ Invalid response, rejected.{}", c.warning, c.reset);
                self.state.reject_pending(last_idx);
                true
            }
        }
    }

    fn execute_action(&mut self, action: &crate::app::core::state::PendingAction) {
        let c = &self.renderer.theme.colors;
        match action {
            crate::app::core::state::PendingAction::Write { path, content } => {
                match crate::app::files::writer::FileWriter::write_file(path, content) {
                    Ok(()) => {
                        logger::success("FileWrite", &format!("{} ({} bytes)", path.display(), content.len()));
                        println!("  {}✓ Wrote {} ({} bytes){}\r\n", c.success, path.display(), content.len(), c.reset);
                    }
                    Err(e) => {
                        logger::error("FileWrite", &format!("Failed: {}", e));
                        println!("  {}✖ Failed to write {}: {}{}\r\n", c.error, path.display(), e, c.reset);
                    }
                }
            }
            crate::app::core::state::PendingAction::Edit { path, old, new } => {
                match crate::app::files::editor::FileEditor::edit_file(path, old, new) {
                    Ok(crate::app::files::editor::EditResult::Applied) => {
                        logger::success("FileEdit", &format!("{}", path.display()));
                        println!("  {}✓ Edited {}{}\r\n", c.success, path.display(), c.reset);
                    }
                    Ok(crate::app::files::editor::EditResult::NotFound) => {
                        logger::warn("FileEdit", &format!("Pattern not found in {}", path.display()));
                        println!("  {}⚠ Pattern not found in {}{}\r\n", c.warning, path.display(), c.reset);
                    }
                    Err(e) => {
                        logger::error("FileEdit", &format!("Failed: {}", e));
                        println!("  {}✖ Failed to edit {}: {}{}\r\n", c.error, path.display(), e, c.reset);
                    }
                }
            }
        }
        print!("\r\n");
        let _ = io::stdout().flush();
    }

    /// Read a single character from the terminal (used for permission prompts).
    fn read_single_char(&self, stdout: &mut io::Stdout) -> Option<String> {
        let _guard = crossterm::terminal::enable_raw_mode().ok()?;
        let result = (|| -> io::Result<Option<String>> {
            loop {
                if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                    if kind != KeyEventKind::Press {
                        continue;
                    }
                    match code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            return Ok(None);
                        }
                        KeyCode::Char('c') => {
                            return Ok(None);
                        }
                        KeyCode::Char(ch) => {
                            write!(stdout, "{}", ch)?;
                            stdout.flush()?;
                            return Ok(Some(ch.to_string()));
                        }
                        KeyCode::Enter => {
                            write!(stdout, "\r\n")?;
                            stdout.flush()?;
                            return Ok(Some(String::new()));
                        }
                        _ => {}
                    }
                }
            }
        })();
        let _ = crossterm::terminal::disable_raw_mode();
        result.ok().flatten()
    }

    fn build_prompt(&self) -> String {
        let c = &self.renderer.theme.colors;
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let short = shorten_path(&cwd, 2);
        format!(
            "{sp}{sp}{dim}{short}{reset}{sp}{bold}{accent}❯{reset}{sp}",
            sp = VISIBLE_SPACE,
            dim = c.dim,
            short = short,
            reset = c.reset,
            bold = c.bold,
            accent = c.accent,
        )
    }

    fn render_welcome(&self, stdout: &mut io::Stdout) {
        let c = &self.renderer.theme.colors;
        let lines = vec![
            format!("{}{}{}  {}✨{}  {}{}Welcome to {}Whyed Code{}{}", c.accent, c.reset, c.bold, c.accent, c.reset, c.bold, c.text, c.accent, "Whyed Code", c.reset),
            format!("  {}{}{}Your AI pair programmer in the terminal.{}{}", c.muted, c.text, c.reset, c.muted, c.reset),
            String::new(),
            format!("  {}{}Model: {} | Version: {}{}", c.muted, c.text, self.state.config.model(), self.state.config.version(), c.reset),
            format!("  {}Permission mode: {:?}{}", c.muted, self.state.permission_mode, c.reset),
            format!("  {}Type {}help{} for commands, or use @file.md to reference files.{}", c.muted, c.accent, c.muted, c.reset),
            String::new(),
        ];
        let _ = self.renderer.render_box(stdout, &lines);
        print!("\r\n");
        let _ = stdout.flush();
    }
}

fn shorten_path(path: &std::path::Path, max_parts: usize) -> String {
    if let Some(home) = dirs_home() {
        if let Ok(rel) = path.strip_prefix(&home) {
            return format!("~/{}", rel.to_string_lossy().replace('\\', "/"));
        }
    }
    let parts: Vec<&std::ffi::OsStr> = path.iter().collect();
    if parts.len() <= max_parts {
        return path.to_string_lossy().replace('\\', "/");
    }
    let tail: Vec<String> = parts[parts.len() - max_parts..]
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    format!(".../{}", tail.join("/"))
}

fn dirs_home() -> Option<std::path::PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(std::path::PathBuf::from)
}
