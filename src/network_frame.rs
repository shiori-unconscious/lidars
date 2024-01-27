/*!
This module contains the implementation of network frames used in the lidar system.
It provides serialization and deserialization functionality for control frames and their associated data structures.
The control frames are used for communication between the lidar device and the control system.
The module also defines the `CmdType` enum, which represents the type of command in a control frame.
The `CmdType` enum has three variants: `Cmd`, `Ack`, and `Msg`.
The module also defines the `Len` trait, which provides a method to calculate the length of a data structure.
The `ControlFrame` struct represents a control frame and contains the necessary fields for serialization and deserialization.
The `ControlFrame` struct is generic over the type of data it contains, which must implement the `Serialize` and `Len` traits.
The module also provides implementations of the `Serialize` and `Deserialize` traits for the `ControlFrame` struct and its associated data structures.
The `Serialize` trait provides a method to serialize a data structure into a byte buffer, while the `Deserialize` trait provides a method to deserialize a byte buffer into a data structure.
The module also defines the `Broadcast` struct, which represents a broadcast frame and contains the necessary fields for serialization and deserialization.
The `Broadcast` struct is used as the data segment in a control frame.
The module also provides implementations of the `Serialize` and `Deserialize` traits for the `Broadcast` struct.
The module also defines the `Cmd` struct, which represents a command frame and contains the necessary fields for serialization and deserialization.
The `Cmd` struct is used as the data segment in a control frame.
The module provides implementations of the `Serialize` and `Deserialize` traits for the `Cmd` struct.
The module also defines constants for the initial values of CRC16 and CRC32, as well as the lengths of various fields in the network frames.
The module imports necessary external crates for CRC calculation and IO operations.
The module is intended to be used as a part of the lidar system for network communication.
*/

use anyhow::{anyhow, Result};
use crc::{self, Crc, CRC_16_MCRF4XX};
use std::io::Write;
use std::mem;

const CRC16_INIT: u16 = 0x4c49;
const CRC32_INIT: u32 = 0x564f580a;
const LEN_OF_LENGTH_FIELD: u16 = 2;
const LEN_OF_CRC16_FIELD: u16 = 2;
const LEN_OF_CRC32_FIELD: u16 = 4;

/// enumeration of command types
#[derive(Clone, Copy)]
pub enum CmdType {
    Cmd,
    Ack,
    Msg,
}
/**
```
let broadcast_frame = ControlFrame::new(CmdType::Cmd, 0x1145, Broadcast::new());
let buffer = Vec::new();
broadcast_frame.serialize(&mut buffer);
```
*/
pub mod control_frame {

    use super::*;

    pub trait Serialize {
        fn serialize<W:Write>(&self, writer: &mut W) -> Result<()>;
    }

    pub trait Deserialize {
        fn deserialize(&mut self, buffer: &[u8]) -> Result<()>;
    }

    pub trait Len {
        fn len() -> u16;
    }

    pub struct Cmd {
        cmd_set: u8,
        cmd_id: u8,
    }

    impl Serialize for Cmd {
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            writer.write_all(&[self.cmd_set, self.cmd_id])?;
            Ok(())
        }
    }

    impl Deserialize for Cmd {
        fn deserialize(&mut self, buffer: &[u8]) -> Result<()> {
            if buffer.len() as u16 != Self::len() {
                return Err(anyhow!(
                    concat!(
                        "Cannot deserialize the serial due to an incompatible length:",
                        "the length of the serial is {}, ", 
                        "while the length of the <Cmd> frame is {}."
                    ),
                    buffer.len(),
                    mem::size_of_val(self),
                ));
            }
            (self.cmd_set, self.cmd_id) = (buffer[0], buffer[1]);
            Ok(())
        }
    }

    impl Len for Cmd {
        fn len() -> u16 {
            return (mem::size_of::<u8>() * 2) as u16;
        }
    }

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
                broadcast_code: [0; 16],
                dev_type: 0x00,
                _reserved: 0x0000,
            }
        }
    }

    impl Serialize for Broadcast {
        fn serialize<W: Write>(&self, writer:&mut W) -> Result<()> {
            self.cmd.serialize(writer)?;
            writer.write_all(&self.broadcast_code)?;
            writer.write_all(&[self.dev_type])?;
            writer.write_all(&self._reserved.to_le_bytes())?;
            Ok(())
        }
    }

    impl Deserialize for Broadcast {
        fn deserialize(&mut self, buffer: &[u8]) -> Result<()> {
            if buffer.len() as u16 != Self::len() {
                return Err(anyhow!(
                    concat!(
                        "Cannot deserialize the serial due to an incompatible length:",
                        "the length of the serial is {}, ",
                        "while the length of the <Broadcast> frame is {}."
                    ),
                    buffer.len(),
                    mem::size_of_val(self),
                ));
            }
            self.cmd.deserialize(&buffer[..2])?;
            self.broadcast_code.copy_from_slice(&buffer[2..18]);
            self.dev_type = buffer[18];
            self._reserved = u16::from_le_bytes(buffer[19..=20].try_into()?);
            Ok(())
        }
    }

    impl Len for Broadcast {
        fn len() -> u16 {
            return (mem::size_of::<u8>() * 17 + mem::size_of::<u16>()) as u16 + Cmd::len();
        }
    }

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

    impl<T> Deserialize for ControlFrame<T>
    where
        T: Deserialize,
    {
        fn deserialize(&mut self, buffer: &[u8]) -> Result<()> {
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
            if digest16.finalize() != u16::from_le_bytes(buffer[7..=8].try_into()?) {
                return Err(anyhow!("Crc16 for header of <ControlFrame> failed"));
            }

            let mut digest32 = crc32fast::Hasher::new_with_initial(CRC32_INIT);
            digest32.update(&buffer[..len - 4]);
            if digest32.finalize() != u32::from_le_bytes(buffer[len - 4..].try_into()?) {
                return Err(anyhow!("Crc32 for frame of <ControlFrame> failed"));
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
            self.data.deserialize(&buffer[9..len - 4])?;

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

    impl<T> ControlFrame<T>
    where
        T: Serialize + Len,
    {
        pub fn serialize(&self) -> Result<Vec<u8>> {
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
            // buf.extend(self.data.serialize()?);
            self.data.serialize(&mut buf)?;
            // calculate CRC32
            digest32.update(&buf);

            buf.extend(digest32.finalize().to_le_bytes());

            Ok(buf)
        }
    }
}
