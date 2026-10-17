use std::hint::black_box;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use todo::display::format_task;
use todo::storage;
use todo::task::{Priority, TodoList};

const SIZES: [usize; 3] = [100, 1_000, 10_000];

fn build_list(size: usize) -> TodoList {
    let priorities = [Priority::Low, Priority::Medium, Priority::High];
    let mut list = TodoList::default();
    for i in 0..size {
        let id = list.add(format!("task number {i}"), priorities[i % 3]);
        if i % 4 == 0 {
            list.find_mut(id).unwrap().done = true;
        }
    }
    list
}

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");
    for size in SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_batched_ref(
                || build_list(size),
                |list| list.add(black_box("new task".into()), Priority::High),
                BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

fn bench_sorted(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorted");
    for size in SIZES {
        let list = build_list(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &list, |b, list| {
            b.iter(|| list.sorted(black_box(false)).len());
        });
    }
    group.finish();
}

fn bench_format(c: &mut Criterion) {
    let list = build_list(1);
    let task = list.sorted(false)[0];
    c.bench_function("format_task/color", |b| {
        b.iter(|| format_task(black_box(task), true));
    });
}

fn bench_storage(c: &mut Criterion) {
    let dir = tempfile::tempdir().unwrap();
    let mut group = c.benchmark_group("storage");
    for size in SIZES {
        let list = build_list(size);
        let path = dir.path().join(format!("todo-{size}.json"));
        storage::save(&path, &list).unwrap();
        group.bench_with_input(BenchmarkId::new("save", size), &list, |b, list| {
            b.iter(|| storage::save(&path, black_box(list)).unwrap());
        });
        group.bench_with_input(BenchmarkId::new("load", size), &path, |b, path| {
            b.iter(|| storage::load(black_box(path)).unwrap());
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_add,
    bench_sorted,
    bench_format,
    bench_storage
);
criterion_main!(benches);
