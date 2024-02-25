use lidar_rs::network_frame::control_frame::{ControlFrame, HANDSHAKE_REQ};
use std::net::UdpSocket;

fn main() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("192.168.1.50:45000")?;
    let control_frame = ControlFrame::new(0x00, HANDSHAKE_REQ);
    let buffer = control_frame.serialize()?;
    let target = "192.168.1.149:65000";
    socket.send_to(&buffer, target)?;
    Ok(())
}
