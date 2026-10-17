use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

struct Sandbox {
    _dir: TempDir,
    file: PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("todo.json");
        Self { _dir: dir, file }
    }

    fn todo(&self, args: &[&str]) -> assert_cmd::assert::Assert {
        Command::new(env!("CARGO_BIN_EXE_todo"))
            .env("TODO_FILE", &self.file)
            .env("NO_COLOR", "1")
            .args(args)
            .assert()
    }
}

#[test]
fn empty_list() {
    Sandbox::new()
        .todo(&["list"])
        .success()
        .stdout("Nenhuma tarefa por aqui.\n");
}

#[test]
fn add_and_list_sorted_by_priority() {
    let s = Sandbox::new();
    s.todo(&["add", "regar", "plantas", "-p", "baixa"])
        .success();
    s.todo(&["add", "estudar", "rust", "-p", "high"])
        .success()
        .stdout(predicate::str::contains("#2 adicionada (prioridade alta)"));

    let out = s.todo(&["list"]).success().get_output().stdout.clone();
    let out = String::from_utf8(out).unwrap();
    let rust = out.find("estudar rust").unwrap();
    let plantas = out.find("regar plantas").unwrap();
    assert!(rust < plantas);
    assert!(out.contains("2 pendente(s), 0 concluída(s)."));
}

#[test]
fn done_undo_and_pending_filter() {
    let s = Sandbox::new();
    s.todo(&["add", "a"]).success();
    s.todo(&["add", "b"]).success();
    s.todo(&["done", "1"]).success();
    s.todo(&["list", "--pending"])
        .success()
        .stdout(predicate::str::contains(" a\n").not())
        .stdout(predicate::str::contains(" b\n"));
    s.todo(&["undo", "1"]).success();
    s.todo(&["list", "-p"])
        .success()
        .stdout(predicate::str::contains(" a\n"));
}

#[test]
fn edit_changes_text_and_priority() {
    let s = Sandbox::new();
    s.todo(&["add", "velho"]).success();
    s.todo(&["edit", "1", "novo", "-p", "alta"]).success();
    s.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("alta  novo"));
}

#[test]
fn edit_without_changes_fails() {
    let s = Sandbox::new();
    s.todo(&["add", "a"]).success();
    s.todo(&["edit", "1"])
        .failure()
        .stderr(predicate::str::contains("informe um novo texto"));
}

#[test]
fn remove_and_clear() {
    let s = Sandbox::new();
    s.todo(&["add", "a"]).success();
    s.todo(&["add", "b"]).success();
    s.todo(&["add", "c"]).success();
    s.todo(&["remove", "2"])
        .success()
        .stdout("Tarefa #2 removida: b\n");
    s.todo(&["done", "3"]).success();
    s.todo(&["clear"])
        .success()
        .stdout("1 tarefa(s) concluída(s) removida(s).\n");
    s.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("1 pendente(s), 0 concluída(s)."));
}

#[test]
fn unknown_id_fails_with_message() {
    Sandbox::new()
        .todo(&["done", "99"])
        .failure()
        .code(1)
        .stderr("erro: tarefa #99 não existe\n");
}

#[test]
fn empty_description_fails() {
    Sandbox::new()
        .todo(&["add", "  "])
        .failure()
        .stderr(predicate::str::contains("precisa de uma descrição"));
}

#[test]
fn corrupted_file_reports_error() {
    let s = Sandbox::new();
    std::fs::write(&s.file, "lixo").unwrap();
    s.todo(&["list"])
        .failure()
        .stderr(predicate::str::contains("corrompido"));
}
