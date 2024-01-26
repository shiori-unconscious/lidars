use criterion::{criterion_group, criterion_main, Criterion};
// use lidar_rs::network_frame::{ControlFrame, BroadCast, CmdType, Serialize}; 
use lidar_rs::network_frame::control_frame::{BroadCast, Cmd, ControlFrame, Serialize, Deserialize};
use lidar_rs::network_frame::CmdType;

fn control_frame_serialize_benchmark(c: &mut Criterion) {
    // 创建 ControlFrame 实例用于测试
    let control_frame = ControlFrame::new(CmdType::Cmd, 0x11, BroadCast{

    });

    c.bench_function("control_frame_serialize", |b| {
        b.iter(|| {
            // 使用黑箱避免编译器优化
            criterion::black_box({
                let mut buffer = Vec::new();
                control_frame.serialize(&mut buffer).unwrap();
            });
        })
    });
}

criterion_group!(benches, control_frame_serialize_benchmark);
criterion_main!(benches);
