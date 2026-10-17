use anyhow::{Result, anyhow, bail};
use clap::Parser;

use todo::cli::{Cli, Command};
use todo::task::TodoList;
use todo::{display, storage};

fn not_found(id: u32) -> anyhow::Error {
    anyhow!("task #{id} does not exist")
}

fn print_list(list: &TodoList, pending_only: bool) {
    let tasks = list.sorted(pending_only);
    if tasks.is_empty() {
        println!("No tasks here.");
        return;
    }
    let color = display::use_color();
    for task in tasks {
        println!("{}", display::format_task(task, color));
    }
    let (pending, done) = list.counts();
    println!("\n{pending} pending, {done} done.");
}

fn run(cli: Cli) -> Result<()> {
    let path = storage::default_path();
    let mut list = storage::load(&path)?;

    match cli.command {
        Command::List { pending } => {
            print_list(&list, pending);
            return Ok(());
        }
        Command::Add { text, priority } => {
            let text = text.join(" ");
            if text.trim().is_empty() {
                bail!("the task needs a description");
            }
            let id = list.add(text, priority);
            println!("Task #{id} added ({} priority).", priority.label());
        }
        Command::Edit { id, text, priority } => {
            let text = text.join(" ");
            let has_text = !text.trim().is_empty();
            if !has_text && priority.is_none() {
                bail!("provide a new text and/or --priority");
            }
            let task = list.find_mut(id).ok_or_else(|| not_found(id))?;
            if has_text {
                task.text = text;
            }
            if let Some(priority) = priority {
                task.priority = priority;
            }
            println!("Task #{id} updated.");
        }
        Command::Done { id } => {
            list.find_mut(id).ok_or_else(|| not_found(id))?.done = true;
            println!("Task #{id} done.");
        }
        Command::Undo { id } => {
            list.find_mut(id).ok_or_else(|| not_found(id))?.done = false;
            println!("Task #{id} moved back to pending.");
        }
        Command::Remove { id } => {
            let task = list.remove(id).ok_or_else(|| not_found(id))?;
            println!("Task #{id} removed: {}", task.text);
        }
        Command::Clear => {
            let removed = list.clear_done();
            println!("{removed} done task(s) removed.");
        }
    }

    storage::save(&path, &list)
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}
