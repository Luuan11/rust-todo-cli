use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::task::TodoList;

pub fn default_path() -> PathBuf {
    if let Some(file) = std::env::var_os("TODO_FILE") {
        return PathBuf::from(file);
    }
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map_or_else(|| PathBuf::from("."), PathBuf::from)
        .join(".todo.json")
}

pub fn load(path: &Path) -> Result<TodoList> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(TodoList::default()),
        Err(e) => {
            return Err(e).with_context(|| format!("could not read {}", path.display()));
        }
    };
    serde_json::from_str(&content).with_context(|| format!("{} is corrupted", path.display()))
}

pub fn save(path: &Path, list: &TodoList) -> Result<()> {
    let json = serde_json::to_string_pretty(list)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json).with_context(|| format!("could not save {}", tmp.display()))?;
    fs::rename(&tmp, path).with_context(|| format!("could not save {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Priority;

    #[test]
    fn missing_file_loads_empty_list() {
        let dir = tempfile::tempdir().unwrap();
        let list = load(&dir.path().join("missing.json")).unwrap();
        assert_eq!(list.counts(), (0, 0));
    }

    #[test]
    fn save_then_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("todo.json");
        let mut list = TodoList::default();
        list.add("a".into(), Priority::High);
        save(&path, &list).unwrap();
        save(&path, &list).unwrap();
        assert_eq!(load(&path).unwrap().counts(), (1, 0));
        assert!(!path.with_extension("json.tmp").exists());
    }

    #[test]
    fn corrupted_file_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("todo.json");
        fs::write(&path, "not json").unwrap();
        let err = load(&path).unwrap_err();
        assert!(err.to_string().contains("is corrupted"));
    }
}
