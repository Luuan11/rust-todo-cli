# 🦀 Rust Todo CLI

📝 A simple and fast task manager for your terminal, written in Rust

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/CLI-4EAA25?style=for-the-badge&logo=gnubash&logoColor=white" alt="CLI" />
  <img src="https://img.shields.io/badge/JSON-5E5C5C?style=for-the-badge&logo=json&logoColor=white" alt="JSON" />
</p>

## ✨ About

A command-line todo list built to practice idiomatic **Rust**. Tasks have priorities, are listed in a smart order, show up with colors in the terminal and are saved to a local JSON file. The project follows Rust best practices: `cargo fmt`, `cargo clippy` (pedantic) with zero warnings, error handling with `anyhow`, small focused modules, unit and integration tests, and benchmarks.

## ✅ Features

- ➕ **Add tasks** with a priority (`low`, `medium`, `high`)
- 📋 **Smart listing** - pending tasks first, then by priority, then by creation order
- ✏️ **Edit** the text and/or the priority of a task
- ✔️ **Done / Undo** - mark a task as completed or move it back to pending
- 🗑️ **Remove** a single task or **clear** all completed ones
- 🎨 **Colored output** - red, yellow and green by priority, dimmed completed tasks
- 🚫 **Respects `NO_COLOR`** and disables colors when the output is not a terminal
- 💾 **Atomic saves** - writes to a temp file and renames it, so a crash never corrupts your list
- 🔁 **Backward compatible** - files saved by older versions still load
- 🧪 **Fully tested** - unit tests and end-to-end tests running the real binary
- ⚡ **Benchmarked** - micro benchmarks with Criterion and real process measurements

## 💡 Technologies

This project was developed with:

- **Rust** (edition 2024)
- **clap** - command-line parsing
- **serde / serde_json** - JSON serialization
- **anyhow** - error handling
- **assert_cmd / predicates / tempfile** - integration tests
- **criterion** - benchmarks

## 🗂️ Project structure

```
src/
  main.rs       # entry point, runs each command
  lib.rs        # exposes the modules to the binary, tests and benchmarks
  cli.rs        # commands and arguments (clap)
  task.rs       # Task, TodoList and Priority (business rules)
  storage.rs    # load and atomic save of the JSON file
  display.rs    # colors and output formatting
tests/
  cli.rs        # end-to-end tests against the compiled binary
benches/
  todo.rs       # Criterion micro benchmarks
bench/
  measure.ps1   # runs the release binary and measures time and memory
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
todo add study rust -p high
todo add water the plants -p low
todo list                             # all tasks
todo list --pending                   # only pending tasks
todo edit 1 buy bread and milk -p low
todo done 2
todo undo 2
todo remove 3
todo clear                            # removes every done task
todo --help
```

```
[ ] #2   high   study rust
[ ] #1   low    buy bread and milk
[ ] #3   low    water the plants
[x] #4   high   pay the bill

3 pending, 1 done.
```

### Storage

Tasks are saved to `~/.todo.json`. Set the `TODO_FILE` environment variable to use another file:

```bash
TODO_FILE=./work.json todo list
```

## ⚡ Benchmarks

Measured on an **AMD Ryzen 7 5700X**, 32 GB RAM, Windows 11, `rustc 1.99.1`, release build (`lto`, `codegen-units = 1`, `strip`).

### Running the binary

Each scenario runs the real `todo.exe` 30 times against a JSON file with N tasks. The time includes everything: starting the process, reading and parsing the file, running the command, saving and exiting.

| Tasks  | Command | Median   | Min      | Peak RAM |
| ------ | ------- | -------- | -------- | -------- |
| 0      | `list`  | 7.99 ms  | 7.21 ms  | 4.98 MB  |
| 0      | `add`   | 9.27 ms  | 8.39 ms  | 4.59 MB  |
| 1,000  | `list`  | 17.05 ms | 15.46 ms | 4.58 MB  |
| 1,000  | `add`   | 14.56 ms | 13.06 ms | 4.59 MB  |
| 1,000  | `done`  | 14.81 ms | 13.79 ms | 4.58 MB  |
| 10,000 | `list`  | 41.73 ms | 39.62 ms | 5.60 MB  |
| 10,000 | `add`   | 18.31 ms | 17.23 ms | 7.46 MB  |
| 10,000 | `done`  | 18.07 ms | 16.75 ms | 7.45 MB  |

**Binary size:** 742 KB, a single executable with no runtime to install.

Even with 10,000 tasks, every command finishes in under 45 ms and uses less than 8 MB of RAM. Most of the cost of small lists is just Windows starting the process. `list` with 10,000 tasks is the slowest case because it formats and prints 10,000 lines.

### Micro benchmarks (Criterion)

| Operation          | 100 tasks | 1,000 tasks | 10,000 tasks |
| ------------------ | --------- | ----------- | ------------ |
| `add`              | 23 ns     | 40 ns       | 95 ns        |
| `sorted` (list)    | 1.3 µs    | 16 µs       | 220 µs       |
| `storage::load`    | 117 µs    | 274 µs      | 2.1 ms       |
| `storage::save`    | 865 µs    | 985 µs      | 2.8 ms       |
| `format_task`      | 352 ns per task (with colors) | | |

`save` costs more than `load` because of the atomic write: the file is written to disk and then renamed.

### Run them yourself

```bash
cargo bench                                                    # Criterion, HTML report in target/criterion
powershell -ExecutionPolicy Bypass -File bench/measure.ps1     # real binary: time, peak RAM, size
```

## 🧪 Tests

```bash
cargo test                                   # unit + integration tests
cargo clippy --all-targets -- -W clippy::pedantic
cargo fmt --check
```

---

Made with 💜 by [Luan Fernando](https://github.com/Luuan11)
