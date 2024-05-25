use intelligent_sight_lib::{
    create_context, create_engine, infer, release_resources, set_input, set_output, SharedBuffer,
    TensorBuffer,
};
use std::sync::Arc;
use std::thread;

#[test]
fn concurrent_rw_test() {
    let shared_buffer =
        Arc::new(SharedBuffer::new(2, || TensorBuffer::new(vec![1, 3, 640, 480])).unwrap());
    let shared_buffer1 = shared_buffer.clone();
    let handle = thread::spawn(move || {
        create_engine("../model.trt", "images", "output0", 640, 480).unwrap();
        create_context().unwrap();
        let mut out_buffer = TensorBuffer::new(vec![1, 32, 6300]).unwrap();
        set_output(&mut out_buffer).unwrap();
        for _ in 0..1000 {
            let mut lock = shared_buffer.read().unwrap();
            set_input(&mut lock).unwrap();
            infer().unwrap();
        }
        release_resources().unwrap();
    });
    for _ in 0..1000 {
        let mut lock = shared_buffer1.write();
        lock.iter_mut().for_each(|num| *num += 12.0);
    }
    handle.join().unwrap();
}
