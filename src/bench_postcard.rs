use serde::{Deserialize, Serialize};
use criterion::{black_box, Criterion};

pub fn bench<T>(c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group("postcard");

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                postcard::to_slice(
                    black_box(&data),
                    black_box(serialize_buffer.as_mut_slice()),
                )
                .unwrap()
            );
        })
    });

    let deserialize_buffer = postcard::to_allocvec(&data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                postcard::from_bytes::<'_, T>(black_box(&deserialize_buffer))
                .unwrap()
            );
        })
    });

    println!("postcard size: {} bytes", deserialize_buffer.len());

    group.finish();
}