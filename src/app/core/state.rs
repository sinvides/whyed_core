use crate::app::config::Config;
use crate::app::history::History;
use crate::app::ai::client::ApiClient;
use serde_json::Value;
use std::path::PathBuf;

/// Global application state shared across all subsystems.
pub struct AppState {
    pub config: Config,
    pub history: History,
    pub api: ApiClient,
    pub messages: Vec<Value>,
    pub cwd: PathBuf,
    pub running: bool,
    /// Permission mode: Ask = prompt before every file change,
    /// AcceptEdits = apply edits automatically, Plan = only show plan.
    pub permission_mode: PermissionMode,
    /// Pending file operations awaiting user confirmation (Ask mode).
    pub pending_actions: Vec<PendingAction>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PermissionMode {
    Ask,
    AcceptEdits,
    Plan,
}

impl Default for PermissionMode {
    fn default() -> Self {
        PermissionMode::Ask
    }
}

#[derive(Debug, Clone)]
pub enum PendingAction {
    Write { path: PathBuf, content: String },
    Edit { path: PathBuf, old: String, new: String },
}

impl AppState {
    pub fn new(config: Config, history: History, api: ApiClient) -> Self {
        Self {
            config,
            history,
            api,
            messages: Vec::new(),
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            running: true,
            permission_mode: PermissionMode::default(),
            pending_actions: Vec::new(),
        }
    }
}
