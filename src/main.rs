// use std::net::UdpSocket;
// use lidar_rs::network_frame::{CmdType, ControlFrame, Cmd};
// use lidar_rs::network_frame::ControlFrame;
// pub struct ShakeCmd {
//     user_ip: u8,
//     data_port: u16,
//     cmd_port: u16,
//     imu_port: u16,
// }



fn main() -> std::io::Result<()> {
    // let socket = UdpSocket::bind("255.255.255.255:55000")?;
    // let socket = UdpSocket::bind("192.168.1.50:45000")?;
    // // socket.set_broadcast(true)?;
    // let mut buffer = Vec::new();
    // let control_frame = ControlFrame::new(CmdType::Cmd, 0x11, Cmd{
    //     cmd_set: 0x00,
    //     cmd_id: 0x02,
    //     cmd_data: (),
    // });
    // control_frame.serialize(&mut buffer)?;
    // let target = "192.168.1.149:65000";
    // socket.send_to(&buffer, target)?;
    Ok(())
}
