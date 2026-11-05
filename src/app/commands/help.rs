use crate::app::core::state::AppState;
use crate::app::commands::CommandHandler;
use crate::app::ui::colors::Colors;
use std::io::Write;

pub struct HelpCommand;

impl CommandHandler for HelpCommand {
    fn name(&self) -> &'static str { "help" }
    fn description(&self) -> &'static str { "Show available commands" }

    fn handle(&self, _state: &mut AppState, _args: &[String]) -> bool {
        let c = Colors::dark();
        println!("\r\n");
        println!("  {}{}▸ Commands{}", c.bold, c.accent, c.reset);
        println!("\r\n");
        println!("    {}/help{}        Show this list", c.accent, c.reset);
        println!("    {}/status{}      Show current configuration", c.accent, c.reset);
        println!("    {}/read <file>{}  Read and show a file", c.accent, c.reset);
        println!("    {}/write <file> <content>{}  Create or overwrite a file", c.accent, c.reset);
        println!("    {}/edit <file> <old> <new>{}  Edit text in a file", c.accent, c.reset);
        println!("    {}/search <pattern> <needle>{}  Search files", c.accent, c.reset);
        println!("    {}/accent <name>{} Change accent color", c.accent, c.reset);
        println!("    {}/clear{}       Clear terminal", c.accent, c.reset);
        println!("    {}/exit{} / {}/quit{}  Exit", c.accent, c.reset, c.accent, c.reset);
        println!("\r\n");
        println!("  {}{}▸ AI chat{}", c.bold, c.secondary, c.reset);
        println!("\r\n");
        println!("    {}Type any message to send it to the AI.{}\r\n", c.muted, c.reset);
        println!("    {}Use @filename to include a file in context, e.g.:@src/main.rs{}\r\n", c.muted, c.reset);
        println!("\r\n");
        let _ = std::io::stdout().flush();
        true
    }
}
