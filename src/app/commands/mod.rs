pub mod help;
pub mod status;
pub mod file_read;
pub mod file_write;
pub mod file_edit;
pub mod file_search;

use crate::app::core::state::AppState;
use crate::app::input::parser::ParsedInput;
use crate::app::core::logger;
use std::io::Write;

pub trait CommandHandler {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn handle(&self, state: &mut AppState, args: &[String]) -> bool;
}

pub struct CommandRegistry {
    pub commands: Vec<Box<dyn CommandHandler>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: vec![
                Box::new(help::HelpCommand),
                Box::new(status::StatusCommand),
                Box::new(file_read::FileReadCommand),
                Box::new(file_write::FileWriteCommand),
                Box::new(file_edit::FileEditCommand),
                Box::new(file_search::FileSearchCommand),
            ]
        }
    }

    pub fn find(&self, name: &str) -> Option<&dyn CommandHandler> {
        self.commands.iter().find(|c| c.name() == name).map(|c| c.as_ref())
    }

    pub fn list(&self) -> Vec<(&'static str, &'static str)> {
        self.commands.iter().map(|c| (c.name(), c.description())).collect()
    }
}

/// Dispatch a parsed input to either a command or the AI.
pub fn dispatch(state: &mut AppState, input: ParsedInput) -> bool {
    match input {
        ParsedInput::Command { name, args } => {
            let registry = CommandRegistry::new();
            if let Some(handler) = registry.find(&name) {
                handler.handle(state, &args)
            } else {
                let c = crate::app::ui::colors::Colors::dark();
                println!("\r\n  {}✖ Unknown command: {}{}\r\n", c.error, name, c.reset);
                println!("\r\n  {}Type {}help{} for available commands.{}\r\n", c.muted, c.accent, c.muted, c.reset);
                true
            }
        }
        ParsedInput::Message { text, file_refs } => {
            if text.trim().is_empty() {
                return true;
            }

            // Handle @file references: read and include them in context
            for file_ref in &file_refs {
                let path = std::path::Path::new(file_ref);
                match crate::app::files::reader::FileReader::read_file(path) {
                    Ok(content) => {
                        logger::info("FileRead", &format!("Included @{} ({} bytes)", file_ref, content.len()));
                        state.messages.push(serde_json::json!({
                            "role": "user",
                            "content": format!("File @{}:\n{}", file_ref, content)
                        }));
                    }
                    Err(e) => {
                        logger::error("FileRead", &format!("Cannot read @{}: {}", file_ref, e));
                    }
                }
            }

            // Add the user message
            state.messages.push(serde_json::json!({"role": "user", "content": text}));

            // Send to AI
            match state.api.chat(&state.messages) {
                Ok(reply) => {
                    state.messages.push(serde_json::json!({"role": "assistant", "content": reply.clone()}));
                    let c = crate::app::ui::colors::Colors::dark();
                    print!("\r\n");
                    print!("  {}{}▸ {}{}{}\r\n", c.secondary, c.accent, c.text, reply, c.reset);
                    print!("\r\n");
                    let _ = io::stdout().flush();
                }
                Err(e) => {
                    let c = crate::app::ui::colors::Colors::dark();
                    let lower = e.to_lowercase();
                    let connection_refused = lower.contains("connection refused")
                        || lower.contains("tcp connect error")
                        || lower.contains("10061");
                    if connection_refused {
                        println!("\r\n  {}✖ Cannot connect to AI server{}", c.error, c.reset);
                        println!("      {}Make sure the server is running on {}{}{}", c.muted, c.text, state.config.ai_url().unwrap_or_default(), c.reset);
                        println!("      {}Expected URL: {}{}/chat/completions{}", c.muted, c.text, state.config.ai_url().unwrap_or_default(), c.reset);
                    } else {
                        println!("\r\n  {}✖ {}{}\r\n", c.error, e, c.reset);
                    }
                    println!("\r\n");
                    let _ = io::stdout().flush();
                }
            }
            true
        }
    }
}

use std::io;
