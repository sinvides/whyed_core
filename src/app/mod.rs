mod ai;
mod commands;
mod config;
mod core;
mod files;
mod history;
mod input;
mod ui;
mod window;

use crate::app::config::Config;
use crate::app::window::TerminalWindow;

pub fn run() {
    let config = Config::new();
    let mut tw = TerminalWindow::new(&config.app_name());
    tw.run();
}
