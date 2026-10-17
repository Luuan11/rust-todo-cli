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
        .stdout("No tasks here.\n");
}

#[test]
fn add_and_list_sorted_by_priority() {
    let s = Sandbox::new();
    s.todo(&["add", "water", "plants", "-p", "baixa"]).success();
    s.todo(&["add", "study", "rust", "-p", "high"])
        .success()
        .stdout(predicate::str::contains("Task #2 added (high priority)"));

    let out = s.todo(&["list"]).success().get_output().stdout.clone();
    let out = String::from_utf8(out).unwrap();
    let rust = out.find("study rust").unwrap();
    let plants = out.find("water plants").unwrap();
    assert!(rust < plants);
    assert!(out.contains("2 pending, 0 done."));
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
    s.todo(&["add", "old"]).success();
    s.todo(&["edit", "1", "new", "-p", "alta"]).success();
    s.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("high   new"));
}

#[test]
fn edit_without_changes_fails() {
    let s = Sandbox::new();
    s.todo(&["add", "a"]).success();
    s.todo(&["edit", "1"])
        .failure()
        .stderr(predicate::str::contains("provide a new text"));
}

#[test]
fn remove_and_clear() {
    let s = Sandbox::new();
    s.todo(&["add", "a"]).success();
    s.todo(&["add", "b"]).success();
    s.todo(&["add", "c"]).success();
    s.todo(&["remove", "2"])
        .success()
        .stdout("Task #2 removed: b\n");
    s.todo(&["done", "3"]).success();
    s.todo(&["clear"])
        .success()
        .stdout("1 done task(s) removed.\n");
    s.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("1 pending, 0 done."));
}

#[test]
fn unknown_id_fails_with_message() {
    Sandbox::new()
        .todo(&["done", "99"])
        .failure()
        .code(1)
        .stderr("error: task #99 does not exist\n");
}

#[test]
fn empty_description_fails() {
    Sandbox::new()
        .todo(&["add", "  "])
        .failure()
        .stderr(predicate::str::contains("needs a description"));
}

#[test]
fn corrupted_file_reports_error() {
    let s = Sandbox::new();
    std::fs::write(&s.file, "garbage").unwrap();
    s.todo(&["list"])
        .failure()
        .stderr(predicate::str::contains("is corrupted"));
}
