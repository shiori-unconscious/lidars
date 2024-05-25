use criterion::{criterion_group, criterion_main, Criterion};
use intelligent_sight_lib::{
    get_image, initialize_camera, uninitialize_camera, FlipFlag, ImageBuffer,
};

fn cam_bench(c: &mut Criterion) {
    let mut buffer_width = vec![0u32; 1];
    let mut buffer_height = vec![0u32; 1];

    if let Err(err) = initialize_camera(1, &mut buffer_width, &mut buffer_height, 1000) {
        panic!(
            "CamThread: Failed to initialize camera with err: {}, retrying...",
            err
        );
    }

    c.bench_function("get frame", |b| {
        let mut image = ImageBuffer::new(buffer_width[0], buffer_height[0]).unwrap();
        b.iter(|| {
            criterion::black_box({
                get_image(0, &mut image, FlipFlag::None).unwrap();
            })
        })
    });

    if let Err(err) = uninitialize_camera() {
        panic!("CamThread: Failed to uninitialize camera with err: {}", err);
    }
}

criterion_group!(benches, cam_bench);
criterion_main!(benches);
