use criterion::{criterion_group, criterion_main, Criterion};
use lidar_rs::network_frame::control_frame::{ControlFrame, SampleCtrlReq, SAMPLE_START_REQ};

fn control_frame_serialize_deserialize_benchmark(c: &mut Criterion) {
    // let read_from = ControlFrame::new(
    //     0x00,
    //     IpConfigReq::new(
    //         0x00,
    //         [192, 168, 1, 150],
    //         [255, 255, 255, 0],
    //         [192, 168, 1, 1],
    //     ),
    // );
    let read_from = ControlFrame::new(0x00, SAMPLE_START_REQ);
    let test_buffer = read_from.serialize().unwrap();

    c.bench_function("control_frame_serialize", |b| {
        b.iter(|| {
            criterion::black_box({
                read_from.serialize().unwrap();
            });
        })
    });

    c.bench_function("control_frame_deserialize", |b| {
        b.iter(|| {
            criterion::black_box({
                ControlFrame::<SampleCtrlReq>::deserialize(&test_buffer).unwrap();
            });
        })
    });

    c.bench_function("control_frame_serialize&deserialize", |b| {
        b.iter(|| {
            criterion::black_box({
                let buffer = read_from.serialize().unwrap();
                ControlFrame::<SampleCtrlReq>::deserialize(&buffer).unwrap();
            });
        })
    });
}

criterion_group!(benches, control_frame_serialize_deserialize_benchmark);
criterion_main!(benches);
