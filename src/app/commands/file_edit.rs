use crate::app::core::state::{AppState, PermissionMode, PendingAction};
use crate::app::commands::CommandHandler;
use crate::app::ui::colors::Colors;
use crate::app::core::logger;
use std::io::Write;
use std::path::Path;

pub struct FileEditCommand;

impl CommandHandler for FileEditCommand {
    fn name(&self) -> &'static str { "edit" }
    fn description(&self) -> &'static str { "Edit text in a file" }

    fn handle(&self, state: &mut AppState, args: &[String]) -> bool {
        let c = Colors::dark();
        if args.len() < 3 {
            println!("\r\n  {}✖ Usage: {}edit <file> <old_text> <new_text>{}\r\n", c.error, c.accent, c.reset);
            return true;
        }
        let path = Path::new(&args[0]);
        let old_text = &args[1];
        let new_text = &args[2];

        let action = PendingAction::Edit {
            path: path.to_path_buf(),
            old: old_text.clone(),
            new: new_text.clone(),
        };

        let allowed = match state.permission_mode {
            PermissionMode::Ask => {
                state.request_permission(action).unwrap_or(false)
            }
            PermissionMode::AcceptEdits => {
                logger::info("FileEdit", &format!("Auto-applied edit to {}", path.display()));
                true
            }
            PermissionMode::Plan => {
                logger::info("FileEdit", &format!("Planned edit to {}", path.display()));
                false
            }
        };

        if allowed {
            match crate::app::files::editor::FileEditor::edit_file(path, old_text, new_text) {
                Ok(crate::app::files::editor::EditResult::Applied) => {
                    logger::success("FileEdit", &format!("Edited {}", path.display()));
                    println!("\r\n  {}✓ Edited {}{}\r\n", c.success, path.display(), c.reset);
                }
                Ok(crate::app::files::editor::EditResult::NotFound) => {
                    logger::warn("FileEdit", &format!("Pattern not found in {}", path.display()));
                    println!("\r\n  {}⚠ Pattern not found in {}{}\r\n", c.warning, path.display(), c.reset);
                }
                Err(e) => {
                    logger::error("FileEdit", &format!("Failed to edit {}: {}", path.display(), e));
                    println!("\r\n  {}✖ Failed to edit {}: {}{}\r\n", c.error, path.display(), e, c.reset);
                }
            }
            println!("\r\n");
            let _ = std::io::stdout().flush();
        }
        true
    }
}
