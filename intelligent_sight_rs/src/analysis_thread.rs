use crate::thread_trait::Processor;
use anyhow::{anyhow, Result};
use intelligent_sight_lib::{
    Detection, DetectionBuffer, Reader, TensorBuffer, UnifiedTrait, Writer,
};
use log::error;
use opencv::{self as cv, core::*};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread::{self, JoinHandle},
};

pub struct AnalysisThread {
    input_buffer: Reader<DetectionBuffer>,
    output_buffer: Writer<Vec<f32>>,
    stop_sig: Arc<AtomicBool>,
}

impl Processor for AnalysisThread {
    type Output = Vec<f32>;

    fn get_output_buffer(&self) -> intelligent_sight_lib::Reader<Self::Output> {
        self.output_buffer.get_reader()
    }

    fn start_processor(self) -> std::thread::JoinHandle<()> {
        thread::spawn(move || {

            while self.stop_sig.load(Ordering::Relaxed) == false {
                let Some(lock_input) = self.input_buffer.read() else {
                    if self.stop_sig.load(Ordering::Relaxed) == false {
                        error!("AnalysisThread: Failed to get input");
                    }
                    break;
                };
                for Detection {
                    conf, cls, points, ..
                } in lock_input.iter()
                {
                    let mut image_points = Vector::<Point2d>::with_capacity(5);
                    for (i, [x, y]) in points.iter().enumerate() {
                        if i != 2 {
                            image_points.push(Point2d::new(*x as f64, (y - 80.0) as f64));
                        }
                    }

                    // 旋转向量和平移向量
                    let mut rvec = Mat::default();
                    let mut tvec = Mat::default();
                    cv::calib3d::solve_pnp(
                        &object_points,
                        &image_points,
                        &camera_matrix,
                        &dist_coeffs,
                        &mut rvec,
                        &mut tvec,
                        false,
                        cv::calib3d::SOLVEPNP_ITERATIVE,
                    )
                    .unwrap();
                }
            }

            self.stop_sig.store(true, Ordering::Relaxed);
        })
    }
}

impl AnalysisThread {
    const POWER_RUNE_WIDTH: f64 = 32.0;
    const POWER_RUNE_HEIGHT: f64 = 10.26;
    const CLASSES: [&'static str; 18] = [
        "PR", "B1", "B2", "B3", "B4", "B5", "BG", "BO", "BB", "R1", "R2", "R3", "R4", "R5", "RG",
        "RO", "RB", "PB",
    ];
    const POWER_RUNE_POINTS: [Point3_<f64>; 4] = [
        Point3d::new(
            Self::POWER_RUNE_WIDTH / 2.0,
            -Self::POWER_RUNE_HEIGHT / 2.0,
            0.0,
        ),
        Point3d::new(
            Self::POWER_RUNE_WIDTH / 2.0,
            Self::POWER_RUNE_HEIGHT / 2.0,
            0.0,
        ),
        Point3d::new(
            -Self::POWER_RUNE_WIDTH / 2.0,
            Self::POWER_RUNE_HEIGHT / 2.0,
            0.0,
        ),
        Point3d::new(
            -Self::POWER_RUNE_WIDTH / 2.0,
            -Self::POWER_RUNE_HEIGHT / 2.0,
            0.0,
        ),
    ];
    #[allow(unused)]
    const COLORS: [VecN<f64, 4>; 5] = [
        VecN::new(0.0, 0.0, 255.0, 255.0),
        VecN::new(0.0, 255.0, 0.0, 255.0),
        VecN::new(255.0, 0.0, 0.0, 255.0),
        VecN::new(255.0, 255.0, 0.0, 255.0),
        VecN::new(255.0, 0.0, 255.0, 255.0),
    ];

    pub fn new(input_buffer: Reader<DetectionBuffer>, stop_sig: Arc<AtomicBool>) -> Result<Self> {
        Ok(AnalysisThread {
            input_buffer,
            output_buffer: Writer::new(4, || Ok(vec![0.0; 100]))?,
            stop_sig,
        })
    }
}
