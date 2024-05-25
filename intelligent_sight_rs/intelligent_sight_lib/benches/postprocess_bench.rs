use criterion::{criterion_group, criterion_main, Criterion};
use intelligent_sight_lib::{
    postprocess, postprocess_destroy, postprocess_init_default, TensorBuffer, UnifiedTrait,
};

fn postprocess_bench(c: &mut Criterion) {
    postprocess_init_default().unwrap();

    let mut input_buffer = TensorBuffer::new(vec![1, 32, 6300]).unwrap();
    input_buffer.iter_mut().for_each(|num| *num = 0.9);
    input_buffer.to_device().unwrap();
    let mut output_buffer = TensorBuffer::new(vec![25, 16]).unwrap();

    c.bench_function("post process", |b| {
        b.iter(|| {
            criterion::black_box({
                postprocess(&mut input_buffer, &mut output_buffer).unwrap();
            })
        })
    });
    postprocess_destroy().unwrap();
}

criterion_group!(benches, postprocess_bench);
criterion_main!(benches);
