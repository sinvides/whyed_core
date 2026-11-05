# Whyed Code

AI pair programmer for your terminal — a Rust CLI that talks to your local AI server.

[English](#english) | [Русский](#русский)

---

## English

### What is Whyed Code?

**Whyed Code** (`whyedcode`) is a terminal-based AI coding companion. It runs in your console, reads your messages, sends them to a local AI server (OpenAI-compatible `/v1/chat/completions`), and prints the assistant's replies. It keeps a conversation history, supports slash commands for configuration, and lets you customize the terminal accent color.

### Features

- 🤖 **AI chat** — sends your input as `user` messages to a local AI server and displays `assistant` replies.
- 💬 **Conversation history** — full message history is sent with each request, so the model remembers context.
- 📜 **Slash commands** — `/help`, `/status`, `/history`, `/ai`, `/accent`, `/clear`, `/exit`, `/quit`.
- 🎨 **Customizable accent** — choose from orange, blue, green, or purple terminal colors.
- 💾 **Persistent history** — session history is saved to `~/.whyed/history.json`.
- 🖥️ **Native Windows binary** — compiled with Rust, no dependencies required to run.

### Quick start

#### Prerequisites

- Rust toolchain (stable) — only needed if you want to build from source.
- A local AI server listening on `http://localhost:8080/v1` (OpenAI-compatible).

#### Build from source

```bash
cargo build --release
```

After building, copy the release binary to the project root:

```bash
copy target/release/whyedcode.exe whyedcode.exe
```

#### Run

```bash
.\whyedcode.exe
```

#### First run

On first launch, `~/.whyed/config.ini` is created automatically:

```ini
[ai]
api = token-abc123
api_url = http://localhost:8080/v1

[app]
accent = orange
version = v2.0.12-dev

[history]
enabled = true
max_items = 100
save_path = ~/.whyed/history.json
```

Edit `api_url` if your AI server runs on a different address.

### Usage

Type anything that doesn't start with `/` to send it to the AI:

```
~/Documents/whyed_core ❯ explain this Rust function
```

Slash commands:

| Command | Description |
|---------|-------------|
| `/help` | Show available commands |
| `/status` | Show current configuration |
| `/history` | Show recent conversation history |
| `/ai` | Show AI settings (key and URL) |
| `/accent` | List available accent colors |
| `/accent <name>` | Change accent color (orange, blue, green, purple) |
| `/clear` | Clear the terminal screen |
| `/exit` / `/quit` | Exit the application |

### Configuration

Config file: `~/.whyed/config.ini` (Windows: `%USERPROFILE%\.whyed\config.ini`).

| Section | Key | Default | Description |
|---------|-----|---------|-------------|
| `[ai]` | `api` | `token-abc123` | Bearer token for the AI server |
| `[ai]` | `api_url` | `http://localhost:8080/v1` | Base URL of your local AI server |
| `[app]` | `accent` | `orange` | Terminal accent color |
| `[app]` | `version` | `v2.0.12-dev` | Application version |
| `[history]` | `enabled` | `true` | Save history to disk |
| `[history]` | `max_items` | `100` | Max history entries kept |
| `[history]` | `save_path` | `~/.whyed/history.json` | History file path |

### Error handling

If the AI server is not running, instead of a raw network error you will see:

```
⚠ Не удалось подключиться к AI-серверу
    Проверьте, что сервер запущен на http://localhost:8080/v1
    URL ожидается: http://localhost:8080/v1/chat/completions
```

### Documentation for AI assistants

See the `.skills/` folder for markdown skill files that describe the project structure, build process, API protocol, and configuration reference for AI coding assistants.

### License

MIT License. See [LICENSE](LICENSE).

---

## Русский

### Что такое Whyed Code?

**Whyed Code** (`whyedcode`) — это AI-партнёр по программированию для терминала. Программа работает в консоли, читает ваши сообщения, отправляет их на локальный сервер ИИ (совместимый с OpenAI `/v1/chat/completions`) и выводит ответы ассистента. Сохраняет историю беседы, поддерживает слэш-команды для настройки и позволяет изменить цвет акцента терминала.

### Возможности

- 🤖 **Чат с ИИ** — отправляет ваш ввод как сообщения `user` на локальный сервер ИИ и отображает ответы `assistant`.
- 💬 **История беседы** — полная история сообщений отправляется с каждым запросом, чтобы модель помнила контекст.
- 📜 **Слэш-команды** — `/help`, `/status`, `/history`, `/ai`, `/accent`, `/clear`, `/exit`, `/quit`.
- 🎨 **Настраиваемый акцент** — выбор из оранжевого, синего, зелёного или фиолетового цветов терминала.
- 💾 **Постоянная история** — история сеанса сохраняется в `~/.whyed/history.json`.
- 🖥️ **Нативный бинарник для Windows** — собран на Rust, дополнительных зависимостей для запуска не требуется.

### Быстрый старт

#### Предварительные требования

- Инструментарий Rust (stable) — только если вы хотите собрать из исходников.
- Локальный сервер ИИ, прослушивающий `http://localhost:8080/v1` (совместимый с OpenAI).

#### Сборка из исходников

```bash
cargo build --release
```

После сборки скопируйте релизный бинарник в корень проекта:

```bash
copy target/release/whyedcode.exe whyedcode.exe
```

#### Запуск

```bash
.\whyedcode.exe
```

#### Первый запуск

При первом запуске автоматически создаётся `~/.whyed/config.ini`:

```ini
[ai]
api = token-abc123
api_url = http://localhost:8080/v1

[app]
accent = orange
version = v2.0.12-dev

[history]
enabled = true
max_items = 100
save_path = ~/.whyed/history.json
```

Отредактируйте `api_url`, если ваш сервер ИИ запущен по другому адресу.

### Использование

Введите любой текст, не начинающийся с `/`, чтобы отправить его ИИ:

```
~/Documents/whyed_core ❯ объясни эту функцию на Rust
```

Слэш-команды:

| Команда | Описание |
|---------|----------|
| `/help` | Показать доступные команды |
| `/status` | Показать текущую конфигурацию |
| `/history` | Показать недавнюю историю беседы |
| `/ai` | Показать настройки ИИ (ключ и URL) |
| `/accent` | Список доступных цветов акцента |
| `/accent <имя>` | Сменить цвет акцента (orange, blue, green, purple) |
| `/clear` | Очистить экран терминала |
| `/exit` / `/quit` | Выйти из приложения |

### Конфигурация

Файл конфигурации: `~/.whyed/config.ini` (Windows: `%USERPROFILE%\.whyed\config.ini`).

| Секция | Ключ | По умолчанию | Описание |
|--------|------|--------------|----------|
| `[ai]` | `api` | `token-abc123` | Токен Bearer для сервера ИИ |
| `[ai]` | `api_url` | `http://localhost:8080/v1` | Базовый URL вашего локального сервера ИИ |
| `[app]` | `accent` | `orange` | Цвет акцента терминала |
| `[app]` | `version` | `v2.0.12-dev` | Версия приложения |
| `[history]` | `enabled` | `true` | Сохранять историю на диск |
| `[history]` | `max_items` | `100` | Максимальное количество записей истории |
| `[history]` | `save_path` | `~/.whyed/history.json` | Путь к файлу истории |

### Обработка ошибок

Если сервер ИИ не запущен, вместо необработанной сетевой ошибки вы увидите:

```
⚠ Не удалось подключиться к AI-серверу
    Проверьте, что сервер запущен на http://localhost:8080/v1
    URL ожидается: http://localhost:8080/v1/chat/completions
```

### Документация для ИИ-ассистентов

В папке `.skills/` находятся markdown-файлы навыков, описывающие структуру проекта, процесс сборки, протокол API и справочник конфигурации для ИИ-помощников по кодированию.

### Лицензия

Лицензия MIT. См. [LICENSE](LICENSE).

---

## Project structure

```
whyed_core/
├── .skills/                    # AI skill documentation (markdown)
│   ├── api-protocol.md
│   ├── build-run.md
│   ├── config.md
│   ├── project.md
│   └── structure.md
├── src/
│   ├── main.rs                 # Entry point
│   └── app/
│       ├── mod.rs              # Module declarations, run()
│       ├── api.rs              # AI HTTP client
│       ├── commands.rs         # Slash commands
│       ├── config.rs           # INI config parser
│       ├── history.rs          # Persistent history
│       ├── input.rs            # Line editor (crossterm)
│       ├── render.rs           # Box rendering
│       ├── ui.rs               # Colors, visible width, padding
│       └── window.rs           # Main REPL loop
├── Cargo.toml                  # Rust manifest
├── Cargo.lock                  # Locked dependencies
├── config.ini                  # Default configuration example
├── whyedcode.exe               # Release binary (root launcher)
├── whyedcode.cmd               # Windows launcher script
├── LICENSE
└── README.md
```

## Contributing

Contributions are welcome! Please open an issue or pull request on [GitHub](https://github.com/sinvides/whyed_core).

## Support

If you encounter issues, please check:
1. That your AI server is running on the configured `api_url`.
2. That the endpoint `<api_url>/chat/completions` is reachable.
3. The `.skills/build-run.md` and `.skills/api-protocol.md` files for troubleshooting details.

---

Made with ❤️ and Rust.
