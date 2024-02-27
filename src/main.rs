use lidar_rs::network_frame::control_frame::{Broadcast, ControlFrame, WriteFlashReq, HANDSHAKE_REQ, HEARTBEAT_REQ};
use std::{net::UdpSocket, thread, time::Duration};
use anyhow::Result;

fn main() -> Result<()> {
    // let socket = UdpSocket::bind("255.255.255.255:55000")?;
    // let mut buffer = vec![0; 100];
    // let (len,_)=socket.recv_from(&mut buffer)?;
    // buffer.truncate(len);
    // match ControlFrame::<Broadcast>::deserialize(&buffer) {
    //     Ok((_, frame)) => println!("{:?}", frame),
    //     Err(e) => println!("err: {e} buffer: {:X?}", buffer),
    // }
    // Ok(())
    let socket = UdpSocket::bind("192.168.1.50:50001")?;
    let control_frame = ControlFrame::new(0, HANDSHAKE_REQ);
    let buffer = control_frame.serialize()?;
    let target = "192.168.1.149:65000";
    socket.send_to(&buffer, target)?;
    let control_frame = ControlFrame::new(0, HEARTBEAT_REQ);    
    let buffer = control_frame.serialize()?;
    loop {
        socket.send_to(&buffer, target)?;
        thread::sleep(Duration::from_secs(2));
    }
    Ok(())
}

