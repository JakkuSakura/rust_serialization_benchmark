use criterion::{black_box, Criterion};

use crate::datasets::page::PAGE_SIZE;
use std::mem::{transmute, transmute_copy};

pub unsafe fn bench<T, R, U>(name: &'static str, c: &mut Criterion, data: &T, read: R, update: U)
where
    R: Fn(&T),
    U: for<'a> Fn(&mut T),
{
    const BUFFER_LEN: usize = PAGE_SIZE;

    let mut group = c.benchmark_group(format!("{}/transmute", name));

    let mut buffer = [0u8; BUFFER_LEN];

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let array: &[u8; PAGE_SIZE] = unsafe { transmute(black_box(data)) };
            black_box(buffer.copy_from_slice(array));
        })
    });
    group.bench_function("read (unvalidated)", |b| {
        b.iter(|| {
            let value: &T = unsafe { transmute(black_box(&buffer)) };
            read(value);
        })
    });
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let value: T = unsafe { transmute_copy(black_box(&buffer)) };
            black_box(value);
        })
    });

    let mut update_buffer = buffer.clone();
    group.bench_function("update (unvalidated)", |b| {
        b.iter(|| {
            let value = unsafe { transmute(black_box(&mut update_buffer)) };
            update(value);
        })
    });

    crate::bench_size(name, "transmute", &buffer);

    group.finish();
}
