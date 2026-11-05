use crate::app::api::ApiClient;
use crate::app::config::Config;
use crate::app::history::History;
use serde_json::json;
use std::io::{self, Write};

pub struct TerminalWindow {
    pub(crate) title: String,
    pub(crate) config: Config,
    pub(crate) history: History,
    pub(crate) api: ApiClient,
    pub(crate) messages: Vec<serde_json::Value>,
    pub(crate) running: bool,
}

impl TerminalWindow {
    pub fn new(title: &str) -> Self {
        let config = Config::new();
        let history_file = config.history_save_path();
        let max_items = config.history_max_items();
        let enabled = config.history_enabled();
        let api = ApiClient::from_config(&config);
        TerminalWindow {
            title: title.to_string(),
            config,
            history: History::new(&history_file, max_items, enabled),
            api,
            messages: Vec::new(),
            running: true,
        }
    }

    pub fn run(&mut self) {
        if let Err(e) = self.clear() {
            eprintln!("warning: failed to clear terminal: {}", e);
        }
        if let Err(e) = self.set_title() {
            eprintln!("warning: failed to set title: {}", e);
        }

        print!("\r\n");
        if let Err(e) = self.render_box(&self.build_content()) {
            eprintln!("warning: failed to render box: {}", e);
        }
        print!("\r\n");
        if let Err(e) = io::stdout().flush() {
            eprintln!("warning: failed to flush stdout: {}", e);
        }

        loop {
            if !self.running {
                break;
            }
            let history_snapshot = self.history.items.clone();
            let input = match self.read_line(&history_snapshot) {
                Ok(Some(s)) => s,
                Ok(None) => {
                    print!("\r\n");
                    self.running = false;
                    break;
                }
                Err(e) => {
                    eprintln!("error: failed to read input: {}", e);
                    self.running = false;
                    break;
                }
            };

            if input.trim().is_empty() {
                continue;
            }

            if let Err(e) = self.history.add(input.trim()) {
                eprintln!("warning: failed to save history: {}", e);
            }

            let should_continue = self.handle_command(&input);
            if !should_continue {
                break;
            }

            if !input.starts_with('/') {
                self.messages.push(json!({"role": "user", "content": input}));
                match self.api.chat(&self.messages) {
                    Ok(reply) => {
                        self.messages.push(json!({"role": "assistant", "content": reply.clone()}));
                        let c = &self.config.colors;
                        print!("\r\n");
                        print!("  {}{}   ▸   {}{}{}\r\n", c.bold, c.accent, c.white, reply, c.reset);
                        print!("\r\n");
                        if let Err(e) = io::stdout().flush() {
                            eprintln!("warning: failed to flush stdout: {}", e);
                        }
                    }
                    Err(e) => {
                        let c = &self.config.colors;
                        print!("\r\n  {}⚠  {}{}\r\n", c.red, e, c.reset);
                        print!("\r\n");
                        if let Err(e) = io::stdout().flush() {
                            eprintln!("warning: failed to flush stdout: {}", e);
                        }
                    }
                }
            }
        }
    }
}
