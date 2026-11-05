use crate::app::core::state::AppState;
use crate::app::commands::CommandHandler;
use crate::app::ui::colors::Colors;
use crate::app::core::logger;
use std::io::Write;
use std::path::Path;

pub struct FileReadCommand;

impl CommandHandler for FileReadCommand {
    fn name(&self) -> &'static str { "read" }
    fn description(&self) -> &'static str { "Read and show a file" }

    fn handle(&self, _state: &mut AppState, args: &[String]) -> bool {
        let c = Colors::dark();
        if args.is_empty() {
            println!("\r\n  {}✖ Usage: {}read <file>{}\r\n", c.error, c.accent, c.reset);
            return true;
        }
        let path = Path::new(&args[0]);
        match crate::app::files::reader::FileReader::read_file_with_lines(path) {
            Ok(lines) => {
                logger::info("FileRead", &format!("{} ({} lines)", path.display(), lines.len()));
                println!("\r\n");
                for (n, line) in lines {
                    let num_str = format!("{:>4}", n);
                    println!("    {}{}{}  {}{}", c.muted, num_str, c.reset, c.text, line);
                }
                println!("\r\n");
            }
            Err(e) => {
                logger::error("FileRead", &format!("Cannot read {}: {}", path.display(), e));
                println!("\r\n  {}✖ Cannot read {}: {}{}\r\n", c.error, path.display(), e, c.reset);
                println!("\r\n");
            }
        }
        let _ = std::io::stdout().flush();
        true
    }
}
