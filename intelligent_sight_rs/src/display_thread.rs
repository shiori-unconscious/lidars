use anyhow::anyhow;
use intelligent_sight_lib::{ImageBuffer, TensorBuffer, UnifiedTrait};
use log::error;
use opencv::{self as cv, core::*};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread::{self, JoinHandle},
};

pub struct DisplayThread {
    image_rx: mpsc::Receiver<ImageBuffer>,
    detection_rx: mpsc::Receiver<TensorBuffer>,
    stop_sig: Arc<AtomicBool>,
}

impl DisplayThread {
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
    const COLORS: [VecN<f64, 4>; 5] = [
        VecN::new(0.0, 0.0, 255.0, 255.0),
        VecN::new(0.0, 255.0, 0.0, 255.0),
        VecN::new(255.0, 0.0, 0.0, 255.0),
        VecN::new(255.0, 255.0, 0.0, 255.0),
        VecN::new(255.0, 0.0, 255.0, 255.0),
    ];

    pub fn new(
        image_rx: mpsc::Receiver<ImageBuffer>,
        detection_rx: mpsc::Receiver<TensorBuffer>,
        stop_sig: Arc<AtomicBool>,
    ) -> Self {
        DisplayThread {
            image_rx,
            detection_rx,
            stop_sig,
        }
    }

    pub fn run(self) -> JoinHandle<()> {
        thread::spawn(move || {
            while self.stop_sig.load(Ordering::Relaxed) == false {
                if let Ok(detection) = self.detection_rx.recv() {
                    let get_image = || loop {
                        match self.image_rx.recv() {
                            Ok(image) => {
                                if image.timestamp == detection.timestamp {
                                    return Ok(image);
                                }
                            }
                            Err(err) => {
                                if self.stop_sig.load(Ordering::Relaxed) == false {
                                    error!("DisplayThread: Failed to get image: {}", err);
                                }
                                return Err(anyhow!("DisplayThread: Failed to get image: {}", err));
                            }
                        }
                    };

                    let res = get_image();
                    if res.is_err() {
                        break;
                    }
                    let mut image = res.unwrap();
                    let mat_ = unsafe {
                        Mat::new_rows_cols_with_data_unsafe(
                            image.height as i32,
                            image.width as i32,
                            CV_8UC3,
                            image.host() as *mut std::ffi::c_void,
                            image.width as usize * 3 * std::mem::size_of::<u8>(),
                        )
                        .unwrap()
                    };
                    let mut mat = Mat::default();
                    cv::imgproc::cvt_color_def(&mat_, &mut mat, cv::imgproc::COLOR_RGB2BGR)
                        .unwrap();
                    let mut iter = detection.iter();

                    for _ in 0..detection.size()[0] {
                        let x = iter.next().unwrap();
                        let y = iter.next().unwrap();
                        let w = iter.next().unwrap();
                        let h = iter.next().unwrap();
                        // println!("{} {} {} {}", x, y, w, h);
                        let conf = iter.next().unwrap();
                        let cls = iter.next().unwrap();
                        // println!("{} {}", conf, cls);
                        if *cls != 0.0 && *cls != 17.0 {
                            continue;
                        }
                        cv::imgproc::circle(
                            &mut mat,
                            cv::core::Point_::new(x.round() as i32, y.round() as i32),
                            5,
                            VecN::new(255.0, 255.0, 255.0, 255.0),
                            -1,
                            0,
                            0,
                        )
                        .unwrap();
                        cv::imgproc::rectangle(
                            &mut mat,
                            Rect_::new(
                                (x - w / 2.0).round() as i32,
                                (y - h / 2.0).round() as i32,
                                w.round() as i32,
                                h.round() as i32,
                            ),
                            VecN::new(255.0, 255.0, 255.0, 255.0),
                            2,
                            0,
                            0,
                        )
                        .unwrap();
                        let cls = match Self::CLASSES.get(*cls as usize) {
                            Some(cls) => cls,
                            None => {
                                self.stop_sig.store(true, Ordering::Relaxed);
                                return;
                            }
                        };
                        cv::imgproc::put_text_def(
                            &mut mat,
                            format!("{} {:.3}", cls, conf).as_str(),
                            Point_::new((x - w / 2.0).round() as i32, (y - h / 2.0).round() as i32),
                            0,
                            0.5,
                            VecN::new(255.0, 255.0, 255.0, 255.0),
                        )
                        .unwrap();
                        let mut image_points = Vector::<Point2d>::with_capacity(5);
                        for i in 0..5 {
                            let x = iter.next().unwrap();
                            let y = iter.next().unwrap();
                            if i != 2 {
                                image_points.push(Point2d::new(*x as f64, *y as f64));
                            }
                            cv::imgproc::circle(
                                &mut mat,
                                cv::core::Point_::new(x.round() as i32, y.round() as i32),
                                5,
                                Self::COLORS[i],
                                -1,
                                0,
                                0,
                            )
                            .unwrap();
                        }

                        // 准备3D点（物体坐标系）
                        let object_points = Vector::from_slice(&Self::POWER_RUNE_POINTS);

                        // 定义相机内参矩阵
                        let camera_matrix = Mat::from_slice_2d(&[
                            [800.0, 0.0, 320.0],
                            [0.0, 800.0, 240.0],
                            [0.0, 0.0, 1.0],
                        ])
                        .unwrap();

                        // 畸变系数（假设无畸变）
                        let dist_coeffs = Mat::zeros(4, 1, cv::core::CV_64F).unwrap();

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

                        let mut image_points = Vector::<Point2d>::with_capacity(4);
                        cv::calib3d::project_points_def(
                            &Vector::from_slice(&[
                                Point3d::new(0.0, 0.0, 0.0),
                                Point3d::new(20.0, 0.0, 0.0),
                                Point3d::new(0.0, 20.0, 0.0),
                                Point3d::new(0.0, 0.0, -20.0),
                            ]),
                            &rvec,
                            &tvec,
                            &camera_matrix,
                            &dist_coeffs,
                            &mut image_points,
                        )
                        .unwrap();
                        let origin = image_points.get(0).unwrap();
                        let origin = Point2i::new(origin.x as i32, origin.y as i32);

                        for i in 0..3 {
                            let pnt = image_points.get(i + 1).unwrap();
                            let pnt = Point2i::new(pnt.x as i32, pnt.y as i32);
                            cv::imgproc::line(&mut mat, origin, pnt, Self::COLORS[i], 2, 0, 0)
                                .unwrap();
                        }
                    }
                    cv::highgui::imshow("Display", &mat).unwrap();
                    cv::highgui::wait_key(10).unwrap();
                }
            }
            cv::highgui::destroy_all_windows().unwrap();
            self.stop_sig.store(true, Ordering::Relaxed);
        })
    }
}
