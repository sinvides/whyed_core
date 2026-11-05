use crate::app::core::state::AppState;
use crate::app::commands::CommandHandler;
use crate::app::ui::colors::Colors;
use std::env;
use std::io::Write;
use std::path::PathBuf;

pub struct StatusCommand;

impl CommandHandler for StatusCommand {
    fn name(&self) -> &'static str { "status" }
    fn description(&self) -> &'static str { "Show current configuration" }
    fn handle(&self, state: &mut AppState, _args: &[String]) -> bool {
        let c = Colors::dark();
        let name = state.config.app_name();
        let version = state.config.version();
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        println!("\r\n");
        println!("  {}{}▸ Status{}", c.bold, c.accent, c.reset);
        println!("\r\n");
        println!("    {}{}{}", c.muted, "Application: ", c.reset);
        println!("    {}", name);
        println!("    {}{}{}", c.muted, "Version: ", c.reset);
        println!("    {}", version);
        println!("    {}{}{}", c.muted, "Permission mode: ", c.reset);
        println!("    {:?}", state.permission_mode);
        println!("    {}{}{}", c.muted, "Directory: ", c.reset);
        println!("    {}", cwd.display());
        println!("\r\n");
        match (&state.config.ai_key(), &state.config.ai_url()) {
            (Some(key), Some(url)) => {
                println!("    {}{}{}", c.muted, "AI: ", c.reset);
                println!("      {}{} = {}{}{}", c.muted, "api", c.reset, mask_key(key), c.reset);
                println!("      {}{} = {}{}{}", c.muted, "api_url", c.reset, url, c.reset);
            }
            _ => {
                println!("    {}No [ai] section{}", c.muted, c.reset);
            }
        }
        println!("\r\n");
        let _ = std::io::stdout().flush();
        true
    }

}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "****".to_string();
    }
    let start = &key[..4];
    let end = &key[key.len() - 4..];
    format!("{}****{}", start, end)
}
