use std::cmp::Reverse;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, ValueEnum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    #[value(alias = "baixa")]
    #[serde(alias = "baixa")]
    Low,
    #[default]
    #[value(alias = "media")]
    #[serde(alias = "media")]
    Medium,
    #[value(alias = "alta")]
    #[serde(alias = "alta")]
    High,
}

impl Priority {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub done: bool,
    #[serde(default)]
    pub priority: Priority,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TodoList {
    next_id: u32,
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn add(&mut self, text: String, priority: Priority) -> u32 {
        self.next_id += 1;
        let id = self.next_id;
        self.tasks.push(Task {
            id,
            text,
            done: false,
            priority,
        });
        id
    }

    pub fn find_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn remove(&mut self, id: u32) -> Option<Task> {
        let pos = self.tasks.iter().position(|t| t.id == id)?;
        Some(self.tasks.remove(pos))
    }

    pub fn clear_done(&mut self) -> usize {
        let before = self.tasks.len();
        self.tasks.retain(|t| !t.done);
        before - self.tasks.len()
    }

    #[must_use]
    pub fn sorted(&self, pending_only: bool) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|t| !pending_only || !t.done)
            .collect();
        tasks.sort_by_key(|t| (t.done, Reverse(t.priority), t.id));
        tasks
    }

    #[must_use]
    pub fn counts(&self) -> (usize, usize) {
        let pending = self.tasks.iter().filter(|t| !t.done).count();
        (pending, self.tasks.len() - pending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assigns_increasing_ids() {
        let mut list = TodoList::default();
        assert_eq!(list.add("a".into(), Priority::Medium), 1);
        assert_eq!(list.add("b".into(), Priority::Medium), 2);
        assert_eq!(list.tasks.len(), 2);
    }

    #[test]
    fn ids_are_not_reused_after_remove() {
        let mut list = TodoList::default();
        list.add("a".into(), Priority::Medium);
        list.remove(1);
        assert_eq!(list.add("b".into(), Priority::Medium), 2);
    }

    #[test]
    fn remove_missing_returns_none() {
        let mut list = TodoList::default();
        assert!(list.remove(42).is_none());
    }

    #[test]
    fn clear_done_keeps_pending() {
        let mut list = TodoList::default();
        list.add("a".into(), Priority::Medium);
        list.add("b".into(), Priority::Medium);
        list.find_mut(1).unwrap().done = true;
        assert_eq!(list.clear_done(), 1);
        assert_eq!(list.tasks.len(), 1);
        assert_eq!(list.tasks[0].id, 2);
    }

    #[test]
    fn counts_pending_and_done() {
        let mut list = TodoList::default();
        list.add("a".into(), Priority::Medium);
        list.add("b".into(), Priority::Medium);
        list.find_mut(2).unwrap().done = true;
        assert_eq!(list.counts(), (1, 1));
    }

    #[test]
    fn json_roundtrip() {
        let mut list = TodoList::default();
        list.add("buy bread".into(), Priority::High);
        let json = serde_json::to_string(&list).unwrap();
        let back: TodoList = serde_json::from_str(&json).unwrap();
        assert_eq!(back.next_id, 1);
        assert_eq!(back.tasks[0].text, "buy bread");
        assert_eq!(back.tasks[0].priority, Priority::High);
    }

    #[test]
    fn priority_is_stored_in_english() {
        let json = serde_json::to_string(&Priority::High).unwrap();
        assert_eq!(json, r#""high""#);
    }

    #[test]
    fn old_portuguese_priorities_still_load() {
        let p: Priority = serde_json::from_str(r#""alta""#).unwrap();
        assert_eq!(p, Priority::High);
        let p: Priority = serde_json::from_str(r#""media""#).unwrap();
        assert_eq!(p, Priority::Medium);
    }

    #[test]
    fn old_files_without_priority_still_load() {
        let json = r#"{"next_id":1,"tasks":[{"id":1,"text":"a","done":false}]}"#;
        let list: TodoList = serde_json::from_str(json).unwrap();
        assert_eq!(list.tasks[0].priority, Priority::Medium);
    }

    #[test]
    fn sorted_puts_pending_and_high_priority_first() {
        let mut list = TodoList::default();
        list.add("low".into(), Priority::Low);
        list.add("high".into(), Priority::High);
        list.add("medium".into(), Priority::Medium);
        list.add("high done".into(), Priority::High);
        list.find_mut(4).unwrap().done = true;
        let ids: Vec<u32> = list.sorted(false).iter().map(|t| t.id).collect();
        assert_eq!(ids, vec![2, 3, 1, 4]);
        let pending: Vec<u32> = list.sorted(true).iter().map(|t| t.id).collect();
        assert_eq!(pending, vec![2, 3, 1]);
    }
}
