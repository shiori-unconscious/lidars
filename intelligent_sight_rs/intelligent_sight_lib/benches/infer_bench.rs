use criterion::{criterion_group, criterion_main, Criterion};
use intelligent_sight_lib::{
    create_context, create_engine, infer, release_resources, set_input, set_output, TensorBuffer,
};

fn infer_bench(c: &mut Criterion) {
    create_engine("../model.trt", "images", "output0", 640, 480).unwrap();
    create_context().unwrap();
    let mut tensor = TensorBuffer::new(vec![640, 480, 3]).unwrap();
    let mut output = TensorBuffer::new(vec![1, 32, 6300]).unwrap();
    set_input(&mut tensor).unwrap();
    set_output(&mut output).unwrap();
    c.bench_function("inference", |b| {
        b.iter(|| {
            criterion::black_box({
                infer().unwrap();
            })
        })
    });
    release_resources().unwrap();
}

criterion_group!(benches, infer_bench);
criterion_main!(benches);
