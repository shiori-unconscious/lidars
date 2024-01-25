// use crc16::{State, ARC, CCITT_FALSE, MCRF4XX};
use byteorder::{LittleEndian, WriteBytesExt};
use crc::{self, Crc, CRC_16_MCRF4XX};
use crc32fast::Hasher;
use std::{mem, net::UdpSocket};

const CRC16_INIT: u16 = 0x4c49;
const CRC32_INIT: u32 = 0x564f580a;

#[derive(Clone, Copy)]
enum CmdType {
    Cmd,
    Ack,
    Msg,
}

trait Serialize {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

struct ControlFrame<T> {
    sof: u8,
    version: u8,
    cmd_type: CmdType,
    seq_num: u16,
    data: T,
}

struct BroadCast;

impl Serialize for BroadCast {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        Ok(())
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

        let mut buf = vec![self.sof, self.version];

        buf.extend((mem::size_of_val(self) as u16).to_le_bytes());

        buf.push(self.cmd_type as u8);

        buf.extend(self.seq_num.to_le_bytes());

        digest16.update(&buf);
        buf.extend(digest16.finalize().to_le_bytes());

        self.data.serialize(&mut buf)?;

        digest32.update(&buf);
        buf.extend(digest32.finalize().to_le_bytes());

        writer.write_all(&buf)?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    // let socket = UdpSocket::bind("255.255.255.255:55000")?;
    // let socket = UdpSocket::bind(addr)
    // socket.set_broadcast(true)?;
    let mut buffer = Vec::new();
    let control_frame = ControlFrame {
        sof: 0xAA,
        version: 0x01,
        cmd_type: CmdType::Cmd,
        seq_num: 0x1,
        data: BroadCast,
    };
    control_frame.serialize(&mut buffer)?;
    // let target = "192.168.1.149:56000";
    // socket.send_to(&buffer, target)?;
    Ok(())
}
