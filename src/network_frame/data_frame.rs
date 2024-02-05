use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DataFrame {
    version: u8,
    slot_id: u8,
    lidar_id: u8,
    reserved: u8,
    status_code: u32,
    timestamp_type: u8,
    data_type: u8,
    timestamp: u64,
    data: Vec<u8>,
}