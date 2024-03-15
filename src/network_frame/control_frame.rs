mod frames;
mod traits;

use anyhow::{anyhow, Result};
use bincode::{deserialize, serialize_into};
use crc::{Crc, CRC_16_MCRF4XX};
use livox_lidar_derive::{CheckStatus, Len};
use serde::{ser::SerializeTupleStruct, Deserialize, Serialize};

use std::mem;

use super::cfg::{CMD_PORT, DATA_PORT, IMU_PORT, USER_IP};

const CRC16_INIT: u16 = 0x9232;
const CRC32_INIT: u32 = 0x564f580a;
pub use frames::*;
pub use traits::*;
