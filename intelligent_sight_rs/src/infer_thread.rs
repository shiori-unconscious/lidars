use crate::thread_trait::Processor;
use anyhow::Result;
use intelligent_sight_lib::{
    convert_rgb888_3dtensor, create_context, create_engine, infer, release_resources, set_input,
    set_output, ImageBuffer, Reader, TensorBuffer, Writer,
};
use log::{debug, error, info, log_enabled, trace};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub struct TrtThread {
    input_buffer: Reader<ImageBuffer>,
    output_buffer: Writer<TensorBuffer>,
    stop_sig: Arc<AtomicBool>,
    #[cfg(feature = "visualize")]
    image_tx: std::sync::mpsc::Sender<ImageBuffer>,
}

impl Drop for TrtThread {
    fn drop(&mut self) {
        if let Err(err) = release_resources() {
            error!("InferThread: Failed to release resources: {}", err);
        }
    }
}

impl Processor for TrtThread {
    type Output = TensorBuffer;

    fn get_output_buffer(&self) -> Reader<Self::Output> {
        self.output_buffer.get_reader()
    }

    fn start_processor(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            if let Err(err) = create_context() {
                error!("InferThread: failed create context, due to error: {}", err);
                self.stop_sig.store(true, Ordering::Relaxed);
                return;
            }

            let mut engine_input_buffer = TensorBuffer::new(vec![1, 3, 640, 480]).unwrap();
            if let Err(err) = set_input(&mut engine_input_buffer) {
                error!("InferThread: set input buffer failed, error {}", err);
                self.stop_sig.store(true, Ordering::Relaxed);
                return;
            }
            info!(
                "InferThread: middle buffer size: {:?}",
                engine_input_buffer.size(),
            );

            let mut cnt = 0;
            let mut start = std::time::Instant::now();
            while self.stop_sig.load(Ordering::Relaxed) == false {
                let Some(mut lock_input) = self.input_buffer.read() else {
                    if self.stop_sig.load(Ordering::Relaxed) == false {
                        error!("InferThread: Failed to get input");
                    }
                    break;
                };
                #[cfg(feature = "visualize")]
                {
                    if let Err(err) = self.image_tx.send(lock_input.clone()) {
                        if self.stop_sig.load(Ordering::Relaxed) == false {
                            error!(
                                "InferThread: send image to display thread failed, error {}",
                                err
                            );
                        }
                        break;
                    }
                }
                if let Err(err) = convert_rgb888_3dtensor(&mut lock_input, &mut engine_input_buffer)
                {
                    error!("InferThread: convert image to tensor failed {}", err);
                    break;
                }

                let timestamp = lock_input.timestamp;
                drop(lock_input);

                if log_enabled!(log::Level::Trace) {
                    trace!("InferThread: finish convert_rgb888_3dtensor");
                }

                let mut lock_output = self.output_buffer.write();
                if let Err(err) = set_output(&mut lock_output) {
                    error!("InferThread: set output buffer failed, error {}", err);
                    break;
                }
                if let Err(err) = infer() {
                    error!("InferThread: infer failed, error {}", err);
                    break;
                }

                lock_output.timestamp = timestamp;
                drop(lock_output);

                if log_enabled!(log::Level::Debug) {
                    cnt += 1;
                    if cnt == 10 {
                        let end = std::time::Instant::now();
                        let elapsed = end.duration_since(start);
                        debug!("InferThread: fps: {}", 10.0 / elapsed.as_secs_f32());
                        start = end;
                        cnt = 0;
                    }
                }
            }
            self.stop_sig.store(true, Ordering::Relaxed);
        })
    }
}

impl TrtThread {
    pub fn new(
        input_buffer: Reader<ImageBuffer>,
        stop_sig: Arc<AtomicBool>,
        #[cfg(feature = "visualize")] image_tx: std::sync::mpsc::Sender<ImageBuffer>,
    ) -> Result<Self> {
        create_engine("model.trt", "images", "output0", 640, 480)?;

        info!("InferThread: output buffer size: {:?}", vec![1, 32, 6300]);
        Ok(Self {
            input_buffer,
            output_buffer: Writer::new(4, || TensorBuffer::new(vec![1, 32, 6300]))?,
            stop_sig,
            #[cfg(feature = "visualize")]
            image_tx,
        })
    }
}
