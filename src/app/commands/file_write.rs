use crate::app::core::state::{AppState, PermissionMode, PendingAction};
use crate::app::commands::CommandHandler;
use crate::app::ui::colors::Colors;
use crate::app::core::logger;
use std::io::Write;
use std::path::Path;

pub struct FileWriteCommand;

impl CommandHandler for FileWriteCommand {
    fn name(&self) -> &'static str { "write" }
    fn description(&self) -> &'static str { "Create or overwrite a file" }

    fn handle(&self, state: &mut AppState, args: &[String]) -> bool {
        let c = Colors::dark();
        if args.len() < 2 {
            println!("\r\n  {}✖ Usage: {}write <file> <content>{}\r\n", c.error, c.accent, c.reset);
            return true;
        }
        let path = Path::new(&args[0]);
        let content = args[1..].join(" ");

        let action = PendingAction::Write {
            path: path.to_path_buf(),
            content: content.clone(),
        };

        let allowed = match state.permission_mode {
            PermissionMode::Ask => {
                state.request_permission(action).unwrap_or(false)
            }
            PermissionMode::AcceptEdits => {
                logger::info("FileWrite", &format!("Auto-applied {}", path.display()));
                true
            }
            PermissionMode::Plan => {
                logger::info("FileWrite", &format!("Planned {}", path.display()));
                false
            }
        };

        if allowed {
            match crate::app::files::writer::FileWriter::write_file(path, &content) {
                Ok(()) => {
                    logger::success("FileWrite", &format!("Wrote {} ({} bytes)", path.display(), content.len()));
                    println!("\r\n  {}✓ Wrote {} ({} bytes){}\r\n", c.success, path.display(), content.len(), c.reset);
                }
                Err(e) => {
                    logger::error("FileWrite", &format!("Failed to write {}: {}", path.display(), e));
                    println!("\r\n  {}✖ Failed to write {}: {}{}\r\n", c.error, path.display(), e, c.reset);
                }
            }
            println!("\r\n");
            let _ = std::io::stdout().flush();
        }
        true
    }
}
