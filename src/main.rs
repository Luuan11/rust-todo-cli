mod cli;
mod display;
mod storage;
mod task;

use anyhow::{Result, anyhow, bail};
use clap::Parser;

use cli::{Cli, Command};
use task::TodoList;

fn not_found(id: u32) -> anyhow::Error {
    anyhow!("tarefa #{id} não existe")
}

fn print_list(list: &TodoList, pending_only: bool) {
    let tasks = list.sorted(pending_only);
    if tasks.is_empty() {
        println!("Nenhuma tarefa por aqui.");
        return;
    }
    let color = display::use_color();
    for task in tasks {
        println!("{}", display::format_task(task, color));
    }
    let (pending, done) = list.counts();
    println!("\n{pending} pendente(s), {done} concluída(s).");
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
                bail!("a tarefa precisa de uma descrição");
            }
            let id = list.add(text, priority);
            println!("Tarefa #{id} adicionada (prioridade {}).", priority.label());
        }
        Command::Edit { id, text, priority } => {
            let text = text.join(" ");
            let has_text = !text.trim().is_empty();
            if !has_text && priority.is_none() {
                bail!("informe um novo texto e/ou --priority");
            }
            let task = list.find_mut(id).ok_or_else(|| not_found(id))?;
            if has_text {
                task.text = text;
            }
            if let Some(priority) = priority {
                task.priority = priority;
            }
            println!("Tarefa #{id} editada.");
        }
        Command::Done { id } => {
            list.find_mut(id).ok_or_else(|| not_found(id))?.done = true;
            println!("Tarefa #{id} concluída.");
        }
        Command::Undo { id } => {
            list.find_mut(id).ok_or_else(|| not_found(id))?.done = false;
            println!("Tarefa #{id} voltou para pendente.");
        }
        Command::Remove { id } => {
            let task = list.remove(id).ok_or_else(|| not_found(id))?;
            println!("Tarefa #{id} removida: {}", task.text);
        }
        Command::Clear => {
            let removed = list.clear_done();
            println!("{removed} tarefa(s) concluída(s) removida(s).");
        }
    }

    storage::save(&path, &list)
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("erro: {e:#}");
        std::process::exit(1);
    }
}
