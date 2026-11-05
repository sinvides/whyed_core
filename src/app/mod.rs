mod api;
mod commands;
mod config;
mod history;
mod input;
mod render;
mod ui;
mod window;

use crate::app::config::Config;
use crate::app::window::TerminalWindow;

pub fn run() {
    let config = Config::new();
    let name = config.app_name();
    let mut tw = TerminalWindow::new(&name);
    tw.run();
}
