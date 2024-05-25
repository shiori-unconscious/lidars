mod analysis_thread;
mod cam_thread;
mod infer_thread;
mod postprocess_thread;
mod thread_trait;

#[cfg(feature = "visualize")]
mod display_thread;

use config::{Config, File};
use env_logger::{Builder, Target};
use log::{error, info};
use serde::Deserialize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use thread_trait::Processor;

#[cfg(feature = "visualize")]
use std::sync::mpsc;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub max_detections: u16,
    pub confidence_threshold: f32,
    pub iou_threshold: f32,
    pub feature_map_size: u16,
    pub camera_exposure_time: u32,
}

static mut CONFIG: Option<AppConfig> = None;

fn set_ctrlc(stop_sig: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        stop_sig.store(true, Ordering::Relaxed);
    })
    .expect("Failed to set Ctrl-C handler");
}

fn main() {
    Builder::from_default_env().target(Target::Stdout).init();

    info!("Main: starting...🚀");

    let Ok(config) = Config::builder()
        .add_source(File::with_name("Config"))
        .build()
    else {
        error!("Failed to load config file");
        return;
    };

    let Ok(config) = config.try_deserialize::<AppConfig>() else {
        error!("Failed to deserialize config");
        return;
    };

    unsafe {
        CONFIG = Some(config);
    }

    let stop_sig = Arc::new(AtomicBool::new(false));

    let camera_thread = match cam_thread::CamThread::new(stop_sig.clone()) {
        Ok(camera_thread) => camera_thread,
        Err(err) => {
            error!("Main: Failed to initialize camera thread: {}", err);
            return;
        }
    };
    info!("Main: Camera thread initialized");

    #[cfg(feature = "visualize")]
    let (tx, rx_img) = mpsc::channel();

    let infer_thread = match infer_thread::TrtThread::new(
        camera_thread.get_output_buffer(),
        stop_sig.clone(),
        #[cfg(feature = "visualize")]
        tx,
    ) {
        Ok(infer_thread) => infer_thread,
        Err(err) => {
            error!("Main: Failed to initialize infer thread: {}", err);
            return;
        }
    };
    info!("Main: Infer thread initialized");

    #[cfg(feature = "visualize")]
    let (tx, rx_det) = mpsc::channel();

    let postprocess_thread = match postprocess_thread::PostprocessThread::new(
        infer_thread.get_output_buffer(),
        stop_sig.clone(),
        #[cfg(feature = "visualize")]
        tx,
    ) {
        Ok(postprocess_thread) => postprocess_thread,
        Err(err) => {
            error!("Main: Failed to initialize postprocess thread: {}", err);
            return;
        }
    };

    let analysis_thread = match analysis_thread::AnalysisThread::new(
        postprocess_thread.get_output_buffer(),
        stop_sig.clone(),
    ) {
        Ok(analysis_thread) => analysis_thread,
        Err(err) => {
            error!("Main: Failed to initialize analysis thread: {}", err);
            return;
        }
    };

    #[cfg(feature = "visualize")]
    let display_thread = display_thread::DisplayThread::new(rx_img, rx_det, stop_sig.clone());

    set_ctrlc(stop_sig.clone());

    let camera_thread_handle = camera_thread.start_processor();
    info!("Main: Camera thread started");

    let infer_thread_handle = infer_thread.start_processor();
    info!("Main: Infer thread started");

    let postprocess_thread_handle = postprocess_thread.start_processor();
    info!("Main: Postprocess thread started");

    let analysis_thread_handle = analysis_thread.start_processor();
    info!("Main: Analysis thread started");

    #[cfg(feature = "visualize")]
    let display_thread_handle = {
        let handle = display_thread.run();
        info!("Main: Display thread started");
        handle
    };

    camera_thread_handle.join().unwrap();
    infer_thread_handle.join().unwrap();
    postprocess_thread_handle.join().unwrap();

    #[cfg(feature = "visualize")]
    display_thread_handle.join().unwrap();

    info!("Main: ending...");
}
