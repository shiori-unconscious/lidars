use criterion::{criterion_group, criterion_main, Criterion};
use intelligent_sight_lib::{ImageBuffer, SharedBuffer};
use std::sync::Arc;
use std::thread;

fn read_write(b1: Arc<SharedBuffer<ImageBuffer>>, b2: Arc<SharedBuffer<ImageBuffer>>) {
    criterion::black_box({
        let h1 = thread::spawn(move || {
            for _ in 0..100 {
                let mut write_buffer = b1.write();
                write_buffer.iter_mut().for_each(|num| *num += 1);
            }
        });
        let h2 = thread::spawn(move || {
            for _ in 0..100 {
                let read_buffer = b2.read();
                read_buffer.iter().for_each(|_| {});
            }
        });
        h1.join().unwrap();
        h2.join().unwrap();
    });
}

fn read_write_bench(c: &mut Criterion) {
    let buffer_size_range = 2..5;
    let buffer_vec: Vec<Arc<SharedBuffer<ImageBuffer>>> = buffer_size_range
        .clone()
        .into_iter()
        .map(|buffer_len| {
            Arc::new(
                SharedBuffer::new(buffer_len, || ImageBuffer::new(1280, 1024))
                    .expect("fail to create sharedbuffer"),
            )
        })
        .collect();

    c.bench_function("just create threads only", |b| {
        b.iter(|| {
            criterion::black_box({
                let h1 = thread::spawn(move || {});
                let h2 = thread::spawn(move || {});
                h1.join().unwrap();
                h2.join().unwrap();
            });
        })
    });

    c.bench_function("allocation bench", |b| {
        b.iter(|| {
            criterion::black_box({
                SharedBuffer::new(2, || ImageBuffer::new(1280, 1024))
                    .expect("fail to allocate sharedbuffer");
            });
        });
    });

    for buffer_size in buffer_size_range.clone() {
        c.bench_function(
            format!(
                "create thread, then read & write for 100 times, buffer size {}",
                buffer_size
            )
            .as_str(),
            |b| {
                b.iter(|| {
                    read_write(
                        buffer_vec[buffer_size - 2].clone(),
                        buffer_vec[buffer_size - 2].clone(),
                    );
                })
            },
        );
    }
}

criterion_group!(benches, read_write_bench);
criterion_main!(benches);
