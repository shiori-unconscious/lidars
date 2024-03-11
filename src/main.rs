use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use livox_lidar_rs::network_frame::cfg::{CMD_PORT, DATA_PORT, USER_IP};
use livox_lidar_rs::network_frame::control_frame::{ControlFrame, HANDSHAKE_REQ};
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::thread;

type JoinHandle = thread::JoinHandle<anyhow::Result<()>>;

struct ThreadManager {
    handle: JoinHandle,
    sig_term_sender: mpsc::Sender<bool>,
}

fn launch_control_thread() -> anyhow::Result<ThreadManager> {
    let broadcast_socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 55000)))?;
    let (_, src) = broadcast_socket.recv_from(&mut Vec::new())?;
    let control_socket = UdpSocket::bind(SocketAddr::from((USER_IP, CMD_PORT)))?;
    let (tx, rx) = mpsc::channel();
    let handle: JoinHandle = thread::spawn(move || {
        let mut seq_num = 0u16;
        control_socket.send_to(&ControlFrame::new(seq_num, HANDSHAKE_REQ).serialize()?, src)?;
        seq_num = seq_num.checked_add(1).unwrap_or_default();
        
        let mut buffer = [0; 1024];
        while rx.try_recv().is_err() {
            let (size, _) = control_socket.recv_from(&mut buffer).unwrap();
            println!("Received {} bytes", size);
        }
        Ok(())
    });
    Ok(ThreadManager {
        handle,
        sig_term_sender: tx,
    })
}
fn main() -> anyhow::Result<()> {
    let control_thread = launch_control_thread()?;
    let data_socket = UdpSocket::bind(SocketAddr::from((USER_IP, DATA_PORT)))?;
    Ok(())
}
