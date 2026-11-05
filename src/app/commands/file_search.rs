use crate::app::core::state::AppState;
use crate::app::commands::CommandHandler;
use crate::app::ui::colors::Colors;
use std::io::Write;
use std::path::Path;

pub struct FileSearchCommand;

impl CommandHandler for FileSearchCommand {
    fn name(&self) -> &'static str { "search" }
    fn description(&self) -> &'static str { "Search files by pattern and content" }

    fn handle(&self, _state: &mut AppState, args: &[String]) -> bool {
        let c = Colors::dark();
        if args.len() < 2 {
            println!("\r\n  {}✖ Usage: {}search <pattern> <needle>{}\r\n", c.error, c.accent, c.reset);
            println!("    {}Example: {}search .rs fn main{}\r\n", c.muted, c.accent, c.reset);
            return true;
        }
        let pattern = &args[0];
        let needle = &args[1];

        match crate::app::files::search::FileSearch::grep(Path::new("."), needle, pattern) {
            Ok(results) => {
                println!("\r\n");
                println!("  {}{}▸ Search results for \"{}\" in *{}{}", c.bold, c.secondary, needle, pattern, c.reset);
                println!("\r\n");
                for m in results.iter().take(30) {
                    let rel = m.path.strip_prefix(".").unwrap_or(&m.path);
                    println!("    {}{}:{}{}", c.accent, rel.display(), m.line_number, c.reset);
                    println!("      {}{}{}", c.muted, m.line, c.reset);
                }
                if results.len() > 30 {
                    println!("    {}{}{}... and {} more{}\r\n", c.muted, c.text, c.reset, results.len() - 30, c.reset);
                } else if results.is_empty() {
                    println!("    {}No matches found.{}\r\n", c.muted, c.reset);
                }
                println!("\r\n");
            }
            Err(e) => {
                println!("\r\n  {}✖ Search failed: {}{}\r\n", c.error, e, c.reset);
                println!("\r\n");
            }
        }
        let _ = std::io::stdout().flush();
        true
    }
}
