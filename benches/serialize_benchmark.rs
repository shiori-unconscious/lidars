use criterion::{criterion_group, criterion_main, Criterion}; 
use lidar_rs::network_frame::control_frame::{Broadcast, ControlFrame, Deserialize};
use lidar_rs::network_frame::CmdType;


fn control_frame_serialize_deserialize_benchmark(c: &mut Criterion) {
    let read_from = ControlFrame::new(CmdType::Cmd, 0x11, Broadcast::new());
    let mut write_to = ControlFrame::new(CmdType::Cmd, 0x11, Broadcast::new());
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
                write_to.deserialize(&test_buffer).unwrap();
            });
        })
    });

    c.bench_function("control_frame_serialize&deserialize", |b| {
        b.iter(|| {
            criterion::black_box({
                let buffer = read_from.serialize().unwrap();
                write_to.deserialize(&buffer).unwrap();
            });
        })
    });

}

criterion_group!(benches, control_frame_serialize_deserialize_benchmark);
criterion_main!(benches);
