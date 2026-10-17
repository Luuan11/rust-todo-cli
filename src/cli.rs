use clap::{Parser, Subcommand};

use crate::task::Priority;

#[derive(Parser)]
#[command(
    name = "todo",
    version,
    about = "Gerenciador de tarefas simples no terminal"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Adiciona uma nova tarefa")]
    Add {
        #[arg(help = "Descrição da tarefa")]
        text: Vec<String>,
        #[arg(
            short,
            long,
            value_enum,
            default_value_t = Priority::Medium,
            help = "Prioridade da tarefa"
        )]
        priority: Priority,
    },
    #[command(about = "Lista as tarefas (pendentes primeiro, por prioridade)")]
    List {
        #[arg(short, long, help = "Mostra só as pendentes")]
        pending: bool,
    },
    #[command(about = "Edita o texto e/ou a prioridade de uma tarefa")]
    Edit {
        id: u32,
        #[arg(help = "Novo texto (opcional)")]
        text: Vec<String>,
        #[arg(short, long, value_enum, help = "Nova prioridade (opcional)")]
        priority: Option<Priority>,
    },
    #[command(about = "Marca uma tarefa como concluída")]
    Done { id: u32 },
    #[command(about = "Volta uma tarefa concluída para pendente")]
    Undo { id: u32 },
    #[command(about = "Remove uma tarefa")]
    Remove { id: u32 },
    #[command(about = "Remove todas as tarefas concluídas")]
    Clear,
}
