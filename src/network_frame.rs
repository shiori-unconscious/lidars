use anyhow::Result;
use crc::{self, Crc, CRC_16_MCRF4XX};
use crc32fast::Hasher;
use std::io::Write;
use std::mem;

const CRC16_INIT: u16 = 0x4c49;
const CRC32_INIT: u32 = 0x564f580a;

#[derive(Clone, Copy)]
pub enum CmdType {
    Cmd,
    Ack,
    Msg,
}
pub mod control_frame {
    use anyhow::anyhow;

    use super::*;

    pub trait Serialize {
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()>;
    }

    pub trait Deserialize {
        fn deserialize(&mut self, buffer: &[u8]) -> Result<()>;
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

    pub struct BroadCast {
        cmd: Cmd,
        broadcast_code: [u8; 16],
        dev_type: u8,
        _reserved: u16,
    }

    // impl Serialize for BroadCast {
    //     fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
    //         Ok(())
    //     }
    // }

    impl Deserialize for Cmd {
        fn deserialize(&mut self, buffer: &[u8]) -> Result<()> {
            if buffer.len() != mem::size_of_val(self) {
                return Err(anyhow!(
                    "Cannot deserialize serial with wrong length: serial length is {}, 
                        while target frame with length {}",
                    buffer.len(),
                    mem::size_of_val(self)
                ));
            }
            (self.cmd_set, self.cmd_id) = (buffer[0], buffer[1]);
            Ok(())
        }
    }

    impl Deserialize for BroadCast {
        fn deserialize(&mut self, buffer: &[u8]) -> Result<()> {
            if buffer.len() != mem::size_of_val(self) {
                return Err(anyhow!(
                    "Cannot deserialize serial with wrong length: serial length is {}, 
                        while target frame with length {}",
                    buffer.len(),
                    mem::size_of_val(self)
                ));
            }
            self.cmd.deserialize(&buffer[..2])?;
            self.broadcast_code.copy_from_slice(&buffer[2..18]);
            self.dev_type = buffer[18];
            Ok(())
        }
    }

    impl Serialize for () {
        fn serialize<W: Write>(&self, _: &mut W) -> Result<()> {
            Ok(())
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

    impl<T> Serialize for ControlFrame<T>
    where
        T: Serialize,
    {
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            let crc16 = Crc::<u16>::new(&CRC_16_MCRF4XX);
            let mut digest16 = crc16.digest_with_initial(CRC16_INIT);
            let mut digest32 = Hasher::new_with_initial(CRC32_INIT);
            let mut buf = Vec::with_capacity(20);

            buf.push(self.sof);
            buf.push(self.version);

            buf.extend((mem::size_of_val(self) as u16).to_le_bytes()); // length

            buf.push(self.cmd_type as u8);

            buf.extend(self.seq_num.to_le_bytes());

            digest16.update(&buf); // calculate CRC16
            buf.extend(digest16.finalize().to_le_bytes());

            // seralize data segment
            self.data.serialize(&mut buf)?;

            digest32.update(&buf); // calculate CRC32
            buf.extend(digest32.finalize().to_le_bytes());

            writer.write_all(&buf)?;
            Ok(())
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
                    "Cannot deserialize serial with wrong length: serial length is {}, 
                        while target frame with length {}",
                    buffer.len(),
                    len,
                ));
            }
            let crc16 = Crc::<u16>::new(&CRC_16_MCRF4XX);
            let mut digest16 = crc16.digest_with_initial(CRC16_INIT);
            digest16.update(&buffer[..7]);
            if digest16.finalize() != u16::from_le_bytes(buffer[7..=8].try_into()?) {
                return Err(anyhow!("Crc16 for header failed"));
            }
            let mut digest32 = Hasher::new_with_initial(CRC32_INIT);
            digest32.update(&buffer[..len - 4]);
            if digest32.finalize() != u32::from_le_bytes(buffer[len - 4..].try_into()?) {
                return Err(anyhow!("Crc32 for data frame failed"));
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
}
