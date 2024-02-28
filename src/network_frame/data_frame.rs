use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize)]
pub struct CartesianPoint {
    x: i32, // millimeters
    y: i32,
    z: i32,
    reflectivity: u8,
    tag: u8,
}

#[derive(Debug, Serialize)]
pub struct SphericalPoint {
    depth: u32, // millimeters
    zenith: u16, // 0.01 degree
    azimuth: u16,
    reflectivity: u8,
    tag: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataFrame<T> {
    version: u8,
    slot_id: u8,
    lidar_id: u8,
    reserved: u8,
    status_code: u32,
    timestamp_type: u8,
    data_type: u8,
    timestamp: u64,
    data: Vec<T>,
}