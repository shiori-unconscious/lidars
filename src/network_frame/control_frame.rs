use anyhow::{anyhow, Result};
use bincode::{deserialize, serialize};
use crc::{self, Crc, CRC_16_MCRF4XX};
use std::mem;

use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

use super::*;
const CRC16_INIT: u16 = 0x9232;
const CRC32_INIT: u32 = 0x501af26a;
const LEN_OF_LENGTH_FIELD: u16 = 2;
const LEN_OF_CRC16_FIELD: u16 = 2;
const LEN_OF_CRC32_FIELD: u16 = 4;

pub trait Len {
    fn len() -> u16;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cmd {
    cmd_set: u8,
    cmd_id: u8,
}

impl Len for Cmd {
    fn len() -> u16 {
        return (mem::size_of::<u8>() * 2) as u16;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Broadcast {
    cmd: Cmd,
    broadcast_code: [u8; 16],
    dev_type: u8,
    _reserved: u16,
}

impl Broadcast {
    pub fn new() -> Self {
        Broadcast {
            cmd: Cmd {
                cmd_set: 0x00,
                cmd_id: 0x00,
            },
            broadcast_code: [1; 16],
            dev_type: 0x00,
            _reserved: 0x1145,
        }
    }
}

impl Len for Broadcast {
    fn len() -> u16 {
        return (mem::size_of::<u8>() * 17 + mem::size_of::<u16>()) as u16 + Cmd::len();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Handshake {
    cmd: Cmd,
    user_ip: Ipv4Addr,
    data_port: u16,
    cmd_port: u16,
    imu_port: u16,
}

impl Handshake {
    pub fn new(user_ip: Ipv4Addr, data_port: u16, cmd_port: u16, imu_port: u16) -> Self {
        Handshake {
            cmd: Cmd {
                cmd_set: 0x00,
                cmd_id: 0x01,
            },
            user_ip,
            data_port,
            cmd_port,
            imu_port,
        }
    }
}

impl Len for Handshake {
    fn len() -> u16 {
        return (mem::size_of::<u8>() * 6 + mem::size_of::<u16>() * 3) as u16 + Cmd::len();
    }
}

pub struct Heartbeat {
    cmd: Cmd,
}

impl Heartbeat {
    fn new() -> Self {
        Heartbeat {
            cmd: Cmd {
                cmd_set: 0x00,
                cmd_id: 0x02,
            },
        }
    }
}

#[derive(Debug)]
pub struct ControlFrame<T> {
    sof: u8,
    version: u8,
    cmd_type: CmdType,
    seq_num: u16,
    data: T,
}

impl<T> ControlFrame<T> {
    pub fn new(cmd_type: CmdType, seq_num: u16, data: T) -> Self {
        ControlFrame {
            sof: 0xAA,
            version: 0x01,
            cmd_type,
            seq_num,
            data,
        }
    }
}

impl<T> ControlFrame<T> {
    pub fn serialize(&self) -> Result<Vec<u8>>
    where
        T: Serialize + Len,
    {
        let crc16 = Crc::<u16>::new(&CRC_16_MCRF4XX);
        let mut digest16 = crc16.digest_with_initial(CRC16_INIT);

        let mut digest32 = crc32fast::Hasher::new_with_initial(CRC32_INIT);

        let buffer_len =
            Self::len() + LEN_OF_LENGTH_FIELD + LEN_OF_CRC16_FIELD + LEN_OF_CRC32_FIELD;

        let mut buf = Vec::with_capacity(buffer_len as usize);

        buf.push(self.sof);
        buf.push(self.version);

        // length of dataframe
        buf.extend((buffer_len).to_le_bytes());
        buf.push(self.cmd_type as u8);

        buf.extend(self.seq_num.to_le_bytes());

        // calculate CRC16
        digest16.update(&buf);
        buf.extend(digest16.finalize().to_le_bytes());

        // seralize data segment
        // self.data.serialize(&mut buf)?;
        buf.extend(
            serialize(&self.data)
                .map_err(|e| anyhow!("Failed to serialize data segment: {}", e))?,
        );

        // calculate CRC32
        digest32.update(&buf);

        buf.extend(digest32.finalize().to_le_bytes());

        Ok(buf)
    }

    pub fn deserialize<'a>(&mut self, buffer: &'a [u8]) -> Result<()>
    where
        T: Deserialize<'a>,
    {
        let len = u16::from_le_bytes(buffer[2..=3].try_into()?) as usize;
        if buffer.len() != len {
            return Err(anyhow!(
                concat!(
                    "Cannot deserialize the serial due to an incompatible length:",
                    "the length of the serial is {}, ",
                    "while the length of the <ControlFrame> frame is {}."
                ),
                buffer.len(),
                len,
            ));
        }

        let crc16 = Crc::<u16>::new(&CRC_16_MCRF4XX);
        let mut digest16 = crc16.digest_with_initial(CRC16_INIT);
        digest16.update(&buffer[..7]);
        let checksum_recv = u16::from_le_bytes(buffer[7..=8].try_into()?);
        let checksum_cal = digest16.finalize();
        if checksum_cal != checksum_recv {
            return Err(anyhow!(
                concat!(
                    "Crc16 for header of <ControlFrame> failed",
                    "checksum received is 0x{:X?}, ",
                    "while the calculated checksum is 0x{:X?}.",
                ),
                checksum_recv,
                checksum_cal
            ));
        }

        let mut digest32 = crc32fast::Hasher::new_with_initial(CRC32_INIT);
        digest32.update(&buffer[..len - 4]);

        let checksum_recv = u32::from_le_bytes(buffer[len - 4..].try_into()?);
        let checksum_cal = digest32.finalize();

        if checksum_cal != checksum_recv {
            return Err(anyhow!(
                concat!(
                    "Crc32 for frame of <ControlFrame> failed",
                    "checksum received is {:X?}, ",
                    "while the calculated checksum is {:X?}.",
                ),
                checksum_recv,
                checksum_cal
            ));
        }

        self.sof = buffer[0];
        self.version = buffer[1];
        self.cmd_type = match buffer[4] {
            0x00 => CmdType::Cmd,
            0x01 => CmdType::Ack,
            0x02 => CmdType::Msg,
            otherwise => return Err(anyhow!("Unknown command type {otherwise}")),
        };
        self.seq_num = u16::from_le_bytes(buffer[5..=6].try_into()?);
        self.data = deserialize(&buffer[9..len - 4])?;
        // self.data.deserialize(&buffer[9..len - 4])?;

        Ok(())
    }
}

impl<T> Len for ControlFrame<T>
where
    T: Len,
{
    fn len() -> u16 {
        return (mem::size_of::<u8>() * 2 + mem::size_of::<CmdType>() + mem::size_of::<u16>())
            as u16
            + Broadcast::len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let control_frame = ControlFrame::new(CmdType::Cmd, 0x11, Broadcast::new());

        let serialized = control_frame.serialize().unwrap();

        assert_eq!(
            serialized,
            vec![
                0xAA, 0x1, 0x22, 0x0, 0x0, 0x11, 0x0, 0xA8, 0x47, 0x0, 0x0, 0x1, 0x1, 0x1, 0x1,
                0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x0, 0x45, 0x11, 0x9E,
                0xBE, 0x83, 0x49
            ]
        );
    }

    #[test]
    fn test_deserialize() {
        let mut control_frame = ControlFrame::new(CmdType::Cmd, 0x0, Broadcast::new());
        let serial = vec![
            0xAA, 0x1, 0x22, 0x0, 0x0, 0x11, 0x0, 0xA8, 0x47, 0x0, 0x0, 0x1, 0x1, 0x1, 0x1, 0x1,
            0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x0, 0x45, 0x11, 0x9E, 0xBE,
            0x83, 0x49,
        ];
        control_frame.deserialize(&serial);
        assert_eq!(control_frame.sof, 0xAA);
        assert_eq!(control_frame.version, 0x1);
        assert_eq!(control_frame.cmd_type, CmdType::Cmd);
        assert_eq!(control_frame.seq_num, 0x11);
    }
}
