use crate::app::window::TerminalWindow;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "****".to_string();
    }
    let start = &key[..4];
    let end = &key[key.len() - 4..];
    format!("{}****{}", start, end)
}

impl TerminalWindow {
    pub fn print_status(&self) {
        let c = &self.config.colors;
        let name = self.config.app_name();
        let accent = self.config.accent();
        let model = self.config.model();
        let version = self.config.version();
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        print!("\r\n");
        print!("  {}{}▸ Статус{}\r\n", c.bold, c.accent, c.reset);
        print!("\r\n");
        print!("    {}Приложение:{}   {}\r\n", c.gray, c.reset, name);
        print!("    {}Модель:{}       {}\r\n", c.gray, c.reset, model);
        print!("    {}Версия:{}       {}\r\n", c.gray, c.reset, version);
        print!("\r\n");
        print!("    {}Акцент:{}       {}{}{}\r\n", c.gray, c.reset, c.accent, accent, c.reset);
        print!("\r\n");
        print!("    {}История:{}      {} записей\r\n", c.gray, c.reset, self.history.items.len());
        print!("    {}Конфиг:{}       {}\r\n", c.gray, c.reset, self.config.path.display());
        print!("    {}Директория:{}   {}\r\n", c.gray, c.reset, cwd.display());
        match (&self.config.ai_key(), &self.config.ai_url()) {
            (Some(key), Some(url)) => {
                print!("\r\n");
                print!("    {}AI:{}\r\n", c.gray, c.reset);
                print!("      {}{} = {}{}{}\r\n", c.gray, "api", c.reset, mask_key(key), c.reset);
                print!("      {}{} = {}{}{}\r\n", c.gray, "api_url", c.reset, url, c.reset);
            }
            _ => {}
        }
        print!("\r\n");
        if let Err(e) = io::stdout().flush() {
            eprintln!("warning: failed to flush stdout: {}", e);
        }
    }

    pub fn print_accents(&self) {
        let c = &self.config.colors;
        print!("\r\n");
        print!("  {}{}▸ Доступные акценты{}\r\n", c.bold, c.accent, c.reset);
        print!("  {}Использование: /accent <имя>{}\r\n", c.gray, c.reset);
        print!("\r\n");
        for (name, acc) in crate::app::config::ACCENTS {
            let mark = if *name == self.config.accent() { "●" } else { " " };
            print!("    {}{} {}{}\r\n", acc.color, mark, name, c.reset);
        }
        print!("\r\n");
        if let Err(e) = io::stdout().flush() {
            eprintln!("warning: failed to flush stdout: {}", e);
        }
    }

    pub fn print_history(&self) {
        let c = &self.config.colors;
        print!("\r\n");
        print!("  {}{}▸ История{}\r\n", c.bold, c.accent, c.reset);
        print!("\r\n");
        if self.history.items.is_empty() {
            print!("    {}Пока пусто{}\r\n", c.gray, c.reset);
        } else {
            let start = self.history.items.len().saturating_sub(20);
            for (i, item) in self.history.items[start..].iter().enumerate() {
                let n = start + i + 1;
                print!("    {}{:>3}{}  {}\r\n", c.gray, n, c.reset, item);
            }
        }
        print!("\r\n");
        if let Err(e) = io::stdout().flush() {
            eprintln!("warning: failed to flush stdout: {}", e);
        }
    }

    pub fn print_ai(&self) {
        let c = &self.config.colors;
        print!("\r\n");
        print!("  {}{}▸ AI настройки{}\r\n", c.bold, c.accent, c.reset);
        print!("\r\n");
        match (&self.config.ai_key(), &self.config.ai_url()) {
            (Some(key), Some(url)) => {
                print!("    {}{} = {}{}{}\r\n", c.gray, "api", c.reset, mask_key(key), c.reset);
                print!("    {}{} = {}{}{}\r\n", c.gray, "api_url", c.reset, url, c.reset);
            }
            _ => {
                print!("    {}Нет секции [ai]{}\r\n", c.gray, c.reset);
            }
        }
        print!("\r\n");
        if let Err(e) = io::stdout().flush() {
            eprintln!("warning: failed to flush stdout: {}", e);
        }
    }

    pub fn set_accent(&mut self, name: &str) {
        if crate::app::config::ACCENTS.iter().any(|(n, _)| *n == name) {
            self.config
                .data
                .entry("app".to_string())
                .or_default()
                .insert("accent".to_string(), name.to_string());
            if let Err(e) = crate::app::config::write_ini(&self.config.path, &self.config.data) {
                let c = &self.config.colors;
                print!("\r\n  {}⚠ Не удалось сохранить конфиг: {}{}\r\n", c.red, e, c.reset);
                print!("\r\n");
                let _ = io::stdout().flush();
                return;
            }
            self.config.refresh_colors();
            let c = &self.config.colors;
            print!("\r\n  {}✓ Акцент изменён на {}{}{}\r\n", c.accent_dim, c.accent, name, c.reset);
            print!("\r\n");
            if let Err(e) = io::stdout().flush() {
                eprintln!("warning: failed to flush stdout: {}", e);
            }
        } else {
            let c = &self.config.colors;
            print!("\r\n  {}✗ Неизвестный акцент: {}{}\r\n", c.red, name, c.reset);
            print!("\r\n");
            if let Err(e) = io::stdout().flush() {
                eprintln!("warning: failed to flush stdout: {}", e);
            }
        }
    }

    pub fn handle_command(&mut self, raw: &str) -> bool {
        let stripped = raw.trim();
        if stripped.is_empty() {
            return true;
        }

        if !stripped.starts_with('/') {
            return true;
        }

        let mut parts = stripped.split_whitespace();
        let cmd = parts.next().unwrap_or("").to_lowercase();

        if cmd == "/exit" || cmd == "/quit" {
            self.running = false;
            return true;
        }

        if cmd == "/clear" {
            if let Err(e) = self.clear() {
                eprintln!("warning: failed to clear terminal: {}", e);
            }
            if let Err(e) = self.set_title() {
                eprintln!("warning: failed to set title: {}", e);
            }
            print!("\r\n");
            if let Err(e) = self.render_box(&self.build_content()) {
                eprintln!("warning: failed to render box: {}", e);
            }
            print!("\r\n");
            if let Err(e) = io::stdout().flush() {
                eprintln!("warning: failed to flush stdout: {}", e);
            }
            return true;
        }

        if cmd == "/status" {
            self.print_status();
            return true;
        }

        if cmd == "/history" {
            self.print_history();
            return true;
        }

        if cmd == "/ai" {
            self.print_ai();
            return true;
        }

        if cmd == "/help" {
            let c = &self.config.colors;
            print!("\r\n");
            print!("  {}{}▸ Команды{}\r\n", c.bold, c.accent, c.reset);
            print!("\r\n");
            print!("    {}/help{}        показать этот список\r\n", c.accent, c.reset);
            print!("    {}/status{}      текущая конфигурация\r\n", c.accent, c.reset);
            print!("    {}/history{}     последние записи истории\r\n", c.accent, c.reset);
            print!("    {}/ai{}          AI настройки из конфига\r\n", c.accent, c.reset);
            print!("    {}/accent{}      список акцентов\r\n", c.accent, c.reset);
            print!("    {}/accent <имя>{} сменить акцент\r\n", c.accent, c.reset);
            print!("    {}/clear{}       очистить экран\r\n", c.accent, c.reset);
            print!("    {}/exit{}        выйти\r\n", c.accent, c.reset);
            print!("\r\n");
            if let Err(e) = io::stdout().flush() {
                eprintln!("warning: failed to flush stdout: {}", e);
            }
            return true;
        }

        if cmd == "/accent" {
            match parts.next() {
                Some(arg) => self.set_accent(&arg.to_lowercase()),
                None => self.print_accents(),
            }
            return true;
        }

        let c = &self.config.colors;
        print!("\r\n  {}Неизвестная команда: {}{}\r\n", c.red, cmd, c.reset);
        print!("\r\n");
        if let Err(e) = io::stdout().flush() {
            eprintln!("warning: failed to flush stdout: {}", e);
        }
        true
    }
}
