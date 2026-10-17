use clap::{Parser, Subcommand};

use crate::task::Priority;

#[derive(Parser)]
#[command(
    name = "todo",
    version,
    about = "A simple task manager for your terminal"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Add a new task")]
    Add {
        #[arg(help = "Task description")]
        text: Vec<String>,
        #[arg(
            short,
            long,
            value_enum,
            default_value_t = Priority::Medium,
            help = "Task priority"
        )]
        priority: Priority,
    },
    #[command(about = "List tasks (pending first, by priority)")]
    List {
        #[arg(short, long, help = "Show only pending tasks")]
        pending: bool,
    },
    #[command(about = "Edit the text and/or the priority of a task")]
    Edit {
        id: u32,
        #[arg(help = "New text (optional)")]
        text: Vec<String>,
        #[arg(short, long, value_enum, help = "New priority (optional)")]
        priority: Option<Priority>,
    },
    #[command(about = "Mark a task as done")]
    Done { id: u32 },
    #[command(about = "Move a done task back to pending")]
    Undo { id: u32 },
    #[command(about = "Remove a task")]
    Remove { id: u32 },
    #[command(about = "Remove all done tasks")]
    Clear,
}
