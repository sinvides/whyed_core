use crate::app::core::state::{AppState, PermissionMode, PendingAction};
use std::io::{self, Write};

impl AppState {
    /// Request permission for a file mutation. Returns true if allowed.
    pub fn request_permission(&mut self, action: PendingAction) -> io::Result<bool> {
        match self.permission_mode {
            PermissionMode::Plan => {
                self.log_plan(&action);
                Ok(false)
            }
            PermissionMode::AcceptEdits => {
                self.log_auto_applied(&action);
                Ok(true)
            }
            PermissionMode::Ask => {
                self.pending_actions.push(action);
                self.print_pending_prompt()?;
                Ok(false) // will be resolved by the UI loop
            }
        }
    }

    pub fn confirm_pending(&mut self, index: usize) -> Option<PendingAction> {
        if index < self.pending_actions.len() {
            Some(self.pending_actions.remove(index))
        } else {
            None
        }
    }

    pub fn reject_pending(&mut self, index: usize) -> Option<PendingAction> {
        if index < self.pending_actions.len() {
            Some(self.pending_actions.remove(index))
        } else {
            None
        }
    }

    pub fn pending_count(&self) -> usize {
        self.pending_actions.len()
    }

    fn log_plan(&self, action: &PendingAction) {
        let c = crate::app::ui::colors::Colors::dark();
        println!("\r\n  {}📋 PLAN{}\r\n", c.accent, c.reset);
        match action {
            PendingAction::Write { path, content } => {
                println!("      {}FileWrite{} {}", c.muted, c.reset, path.display());
                println!("      {}Content length:{} {} bytes", c.muted, c.reset, content.len());
            }
            PendingAction::Edit { path, old, new } => {
                println!("      {}FileEdit{} {}", c.muted, c.reset, path.display());
                println!("      {}Old length:{} {} bytes", c.muted, c.reset, old.len());
                println!("      {}New length:{} {} bytes", c.muted, c.reset, new.len());
            }
        }
        println!("\r\n  {}Plan mode: no changes applied.{}", c.warning, c.reset);
        println!("\r\n");
        let _ = io::stdout().flush();
    }

    fn log_auto_applied(&self, action: &PendingAction) {
        let c = crate::app::ui::colors::Colors::dark();
        println!("\r\n  {}✓ Auto-applied{}", c.success, c.reset);
        match action {
            PendingAction::Write { path, .. } => {
                println!("      {}FileWrite{} {}", c.muted, c.reset, path.display());
            }
            PendingAction::Edit { path, .. } => {
                println!("      {}FileEdit{} {}", c.muted, c.reset, path.display());
            }
        }
        println!("\r\n");
        let _ = io::stdout().flush();
    }

    fn print_pending_prompt(&self) -> io::Result<()> {
        let c = crate::app::ui::colors::Colors::dark();
        if self.pending_actions.is_empty() {
            return Ok(());
        }
        let last = self.pending_actions.last().unwrap();
        println!("\r\n  {}⚠ Permission required{}", c.warning, c.reset);
        match last {
            PendingAction::Write { path, .. } => {
                println!("      {}FileWrite{} {}", c.muted, c.reset, path.display());
            }
            PendingAction::Edit { path, .. } => {
                println!("      {}FileEdit{} {}", c.muted, c.reset, path.display());
            }
        }
        println!("      {}(a)llow  (n)one  (p)lan mode{}", c.muted, c.reset);
        print!("      {}> {} ", c.accent, c.reset);
        io::stdout().flush()
    }
}
