use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use livox_lidar_rs::network_frame::cfg::{CMD_PORT, DATA_PORT, USER_IP};
use livox_lidar_rs::network_frame::control_frame::{ControlFrame, HANDSHAKE_REQ};
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::sync::mpsc;

fn init_control_thread() -> anyhow::Result<(mpsc::Sender<bool>, thread::JoinHandle<Result<(), anyhow::Error>>)> {
    let broadcast_socket = UdpSocket::bind(SocketAddr::from((USER_IP, 55000)))?;
    let (_, src) = broadcast_socket.recv_from(&mut Vec::new())?;
    let control_socket = UdpSocket::bind(SocketAddr::from((USER_IP, CMD_PORT)))?;
    let (tx, rx) = mpsc::channel();
    let handle: thread::JoinHandle<Result<(), anyhow::Error>> = thread::spawn(move || {
        control_socket.send_to(&ControlFrame::new(0x00, HANDSHAKE_REQ).serialize()?, src)?;
        let mut buffer = [0; 1024];
        if rx.try_recv().is_err() {
            let (size, _) = control_socket.recv_from(&mut buffer).unwrap();
            println!("Received {} bytes", size);
        }
        Ok(())
    });
    Ok((tx, handle))
}
fn main() -> anyhow::Result<()> {
    
    let (sender, control_handle) = init_control_thread()?;
    let data_socket = UdpSocket::bind(SocketAddr::from((USER_IP, DATA_PORT)))?;    
    Ok(())
}
