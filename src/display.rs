use std::io::IsTerminal;

use crate::task::{Priority, Task};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";

#[must_use]
pub fn use_color() -> bool {
    std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none()
}

fn priority_color(priority: Priority) -> &'static str {
    match priority {
        Priority::Low => GREEN,
        Priority::Medium => YELLOW,
        Priority::High => RED,
    }
}

fn paint(text: &str, code: &str, color: bool) -> String {
    if color {
        format!("{code}{text}{RESET}")
    } else {
        text.to_string()
    }
}

#[must_use]
pub fn format_task(task: &Task, color: bool) -> String {
    let mark = if task.done { "x" } else { " " };
    let priority = format!("{:<6}", task.priority.label());
    if task.done {
        let line = format!("[{mark}] #{:<3} {priority} {}", task.id, task.text);
        return paint(&line, DIM, color);
    }
    format!(
        "[{mark}] {} {} {}",
        paint(&format!("#{:<3}", task.id), BOLD, color),
        paint(&priority, priority_color(task.priority), color),
        task.text
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(done: bool) -> Task {
        Task {
            id: 7,
            text: "x".into(),
            done,
            priority: Priority::High,
        }
    }

    #[test]
    fn without_color_has_no_escape_codes() {
        let s = format_task(&task(false), false);
        assert!(!s.contains('\x1b'));
        assert!(s.contains("#7") && s.contains("high"));
    }

    #[test]
    fn pending_task_uses_priority_color() {
        assert!(format_task(&task(false), true).contains(RED));
    }

    #[test]
    fn done_task_is_dimmed() {
        let s = format_task(&task(true), true);
        assert!(s.starts_with(DIM) && s.contains("[x]"));
    }
}
