use crate::{thread_trait::Processor, CONFIG};
use anyhow::{anyhow, Result};

use intelligent_sight_lib::{ImageBuffer, Reader, Writer};

#[allow(unused)]
use log::{debug, error, info, log_enabled};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{path, thread};

#[cfg(feature = "from_video")]
use opencv::{self as cv, prelude::*};

#[cfg(not(feature = "from_video"))]
use intelligent_sight_lib::{get_image, initialize_camera, uninitialize_camera, FlipFlag};

pub struct CamThread {
    shared_buffer: Writer<ImageBuffer>,
    stop_sig: Arc<AtomicBool>,
    #[cfg(feature = "from_video")]
    video_capture: cv::videoio::VideoCapture,
}

impl CamThread {
    pub fn new(stop_sig: Arc<AtomicBool>) -> Result<Self> {
        #[cfg(not(feature = "from_video"))]
        {
            let mut buffer_width = vec![0u32; 1];
            let mut buffer_height = vec![0u32; 1];
            unsafe {
                if let Some(Some(config)) = std::ptr::addr_of!(CONFIG).as_ref() {
                    initialize_camera(
                        1,
                        &mut buffer_width,
                        &mut buffer_height,
                        config.camera_exposure_time,
                    )?
                } else {
                    initialize_camera(1, &mut buffer_width, &mut buffer_height, 4000)?
                }
            };

            info!(
                "CamThread: Camera initialized, width: {}, height: {}",
                buffer_width[0], buffer_height[0]
            );
            Ok(CamThread {
                shared_buffer: Writer::new(4, || {
                    ImageBuffer::new(buffer_width[0], buffer_height[0])
                })?,
                stop_sig,
            })
        }
        #[cfg(feature = "from_video")]
        {
            let args = std::env::args().collect::<Vec<_>>();
            let path = if args.len() > 1 {
                let path_buf = path::Path::new("testvideos").join(&args[1]);
                path_buf
            } else {
                path::Path::new("testvideos").join("power_rune.mp4")
            };
            Ok(CamThread {
                shared_buffer: Writer::new(4, || ImageBuffer::new(640, 480))?,
                stop_sig,
                video_capture: cv::videoio::VideoCapture::from_file(
                    path.to_str().unwrap(),
                    cv::videoio::CAP_FFMPEG,
                )
                .map_err(|e| anyhow!("Failed to open video file: {}", e))?,
            })
        }
    }
}

#[cfg(not(feature = "from_video"))]
impl Drop for CamThread {
    fn drop(&mut self) {
        let mut uninitialize_retry = 0;
        while let Err(err) = uninitialize_camera() {
            error!(
                "CamThread: Failed to uninitialize camera with err: {}, retrying...",
                err
            );
            uninitialize_retry += 1;
            if uninitialize_retry > 10 {
                error!("CamThread: Failed to uninitialize camera after 10 retries, exiting...");
            }
        }
    }
}

impl Processor for CamThread {
    type Output = ImageBuffer;

    fn get_output_buffer(&self) -> Reader<ImageBuffer> {
        self.shared_buffer.get_reader()
    }

    fn start_processor(mut self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut cnt = 0;
            let mut start = std::time::Instant::now();

            while self.stop_sig.load(Ordering::Relaxed) == false {
                let mut lock = self.shared_buffer.write();
                #[cfg(not(feature = "from_video"))]
                if let Err(err) = get_image(0, &mut lock, FlipFlag::None) {
                    error!("err: {}", err);
                    break;
                }
                #[cfg(feature = "from_video")]
                {
                    let mut frame = cv::core::Mat::default();
                    if !self.video_capture.read(&mut frame).unwrap() {
                        break;
                    }
                    let mut mat = cv::core::Mat::default();
                    cv::imgproc::resize(
                        &frame,
                        &mut mat,
                        cv::core::Size::new(lock.width as i32, lock.height as i32),
                        0.0,
                        0.0,
                        cv::imgproc::INTER_LINEAR,
                    )
                    .unwrap();
                    let mut dst = Mat::default();
                    cv::imgproc::cvt_color_def(&mat, &mut dst, cv::imgproc::COLOR_BGR2RGB).unwrap();
                    dst.data_bytes()
                        .unwrap()
                        .iter()
                        .zip(lock.iter_mut())
                        .for_each(|(a, b)| {
                            *b = a.checked_mul(1).unwrap_or(255);
                        });
                }

                // TODO: get timestamp from camera
                lock.timestamp = std::time::Instant::now();
                drop(lock);

                cnt += 1;
                if cnt == 10 {
                    let end = std::time::Instant::now();
                    let elapsed = end.duration_since(start);
                    if log_enabled!(log::Level::Debug) {
                        debug!("CamThread: fps: {}", 10.0 / elapsed.as_secs_f32());
                    }
                    start = end;
                    cnt = 0;
                }
            }
            self.stop_sig.store(true, Ordering::Relaxed);
        })
    }
}
