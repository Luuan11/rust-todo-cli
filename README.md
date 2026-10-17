# 🦀 Rust Todo CLI

📝 A simple and fast task manager for your terminal, written in Rust

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/CLI-4EAA25?style=for-the-badge&logo=gnubash&logoColor=white" alt="CLI" />
  <img src="https://img.shields.io/badge/JSON-5E5C5C?style=for-the-badge&logo=json&logoColor=white" alt="JSON" />
</p>

## ✨ About

A command-line todo list built to practice idiomatic **Rust**. Tasks have priorities, are listed in a smart order, show up with colors in the terminal and are saved to a local JSON file. The project follows Rust best practices: `cargo fmt`, `cargo clippy` (pedantic) with zero warnings, error handling with `anyhow`, small focused modules, and both unit and integration tests.

## ✅ Features

- ➕ **Add tasks** with a priority (`baixa`, `media`, `alta`, or `low`, `medium`, `high`)
- 📋 **Smart listing** - pending tasks first, then by priority, then by creation order
- ✏️ **Edit** the text and/or the priority of a task
- ✔️ **Done / Undo** - mark a task as completed or move it back to pending
- 🗑️ **Remove** a single task or **clear** all completed ones
- 🎨 **Colored output** - red, yellow and green by priority, dimmed completed tasks
- 🚫 **Respects `NO_COLOR`** and disables colors when the output is not a terminal
- 💾 **Atomic saves** - writes to a temp file and renames it, so a crash never corrupts your list
- 🔁 **Backward compatible** - files saved by older versions still load
- 🧪 **Fully tested** - unit tests and end-to-end tests running the real binary

## 💡 Technologies

This project was developed with:

- **Rust** (edition 2024)
- **clap** - command-line parsing
- **serde / serde_json** - JSON serialization
- **anyhow** - error handling
- **assert_cmd / predicates / tempfile** - integration tests

## 🗂️ Project structure

```
src/
  main.rs       # entry point, runs each command
  cli.rs        # commands and arguments (clap)
  task.rs       # Task, TodoList and Priority (business rules)
  storage.rs    # load and atomic save of the JSON file
  display.rs    # colors and output formatting
tests/
  cli.rs        # end-to-end tests against the compiled binary
```

## 📦 Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) 1.85 or higher

### Step by Step

1. **Clone the repository**
```bash
git clone https://github.com/Luuan11/rust-todo-cli.git
cd rust-todo-cli
```

2. **Run it**
```bash
cargo run -- list
```

3. **Or install it globally** to use `todo` from anywhere
```bash
cargo install --path .
```

## 🔧 Usage

```bash
todo add buy bread                    # medium priority by default
todo add study rust -p alta           # high priority
todo add water the plants -p low      # english names work too
todo list                             # all tasks
todo list --pending                   # only pending tasks
todo edit 1 buy bread and milk -p baixa
todo done 2
todo undo 2
todo remove 3
todo clear                            # removes every completed task
todo --help
```

```
[ ] #2   alta  study rust
[ ] #1   baixa buy bread and milk
[ ] #3   baixa water the plants
[x] #4   alta  pay the bill

3 pendente(s), 1 concluída(s).
```

### Storage

Tasks are saved to `~/.todo.json`. Set the `TODO_FILE` environment variable to use another file:

```bash
TODO_FILE=./work.json todo list
```

## 🧪 Tests

```bash
cargo test                                   # unit + integration tests
cargo clippy --all-targets -- -W clippy::pedantic
cargo fmt --check
```

---

Made with 💜 by [Luan Fernando](https://github.com/Luuan11)
