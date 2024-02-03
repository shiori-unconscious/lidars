use lidar_rs::network_frame::control_frame::{ControlFrame, WriteFlashReq};
use std::net::UdpSocket;

fn main() {
    let read_from = ControlFrame::new(0x11, WriteFlashReq::new(false, true, 0x08));
    // let mut write_to = ControlFrame::new(CmdType::Cmd, 0x11, Broadcast::new());
    // let test_buffer = read_from.serialize().unwrap();

    println!("{:?}", read_from);

    let buffer = read_from.serialize().unwrap();
    println!("{:X?}", buffer);
    // write_to.deserialize(&buffer).unwrap();
    // println!("{:?}",write_to);
}
// fn main() -> Result<()> {
//     let socket = UdpSocket::bind("255.255.255.255:55000")?;
//     let mut buffer = vec![0; 100];
//     let (len,_)=socket.recv_from(&mut buffer)?;
//     buffer.truncate(len);
//     let mut control_frame = ControlFrame::new(CmdType::Cmd, 0x11, Broadcast::new());
//     match control_frame.deserialize(&buffer) {
//         Ok(_) => println!("{:?}", control_frame),
//         Err(e) => println!("err: {e} buffer: {:X?}", buffer),
//     }
//     // let socket = UdpSocket::bind("192.168.1.50:45000")?;
//     // let mut buffer = Vec::new();
//     // let control_frame = ControlFrame::new(CmdType::Cmd, 0x11, Cmd{
//     //     cmd_set: 0x00,
//     //     cmd_id: 0x02,
//     //     cmd_data: (),
//     // });
//     // control_frame.serialize(&mut buffer)?;
//     // let target = "192.168.1.149:65000";
//     // socket.send_to(&buffer, target)?;
//     Ok(())
// }
