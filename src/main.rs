use std::net::UdpSocket;
use lidar_rs::network_frame::{BroadCast, ControlFrame, CmdType, Serialize};

fn main() -> std::io::Result<()> {
    // let socket = UdpSocket::bind("255.255.255.255:55000")?;
    let socket = UdpSocket::bind("192.168.1.50:45000")?;
    // socket.set_broadcast(true)?;
    let mut buffer = Vec::new();
    let control_frame = ControlFrame::new(CmdType::Cmd, 0x11, BroadCast);
    control_frame.serialize(&mut buffer)?;
    let target = "192.168.1.149:65000";
    socket.send_to(&buffer, target)?;
    Ok(())
}
