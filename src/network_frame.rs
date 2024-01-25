use crc::{self, Crc, CRC_16_MCRF4XX};
use crc32fast::Hasher;
use std::mem;

const CRC16_INIT: u16 = 0x4c49;
const CRC32_INIT: u32 = 0x564f580a;

#[derive(Clone, Copy)]
pub enum CmdType {
    Cmd,
    Ack,
    Msg,
}

pub struct BroadCast;

impl Serialize for BroadCast {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        Ok(())
    }
}

pub trait Serialize {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

// pub trait Deserialize {
//     fn deserialize(&self) -> 
// }

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
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
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
