use anyhow::Ok;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use livox_lidar_rs::network_frame::cfg::{CMD_PORT, DATA_PORT, USER_IP};
use livox_lidar_rs::network_frame::control_frame::{
    CheckStatus, ControlFrame, HandshakeResp, HANDSHAKE_REQ,
};
use log::{debug, info, log_enabled, warn};
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
type JoinHandle = thread::JoinHandle<anyhow::Result<()>>;

struct ThreadManager {
    handle: JoinHandle,
    sig_term_sender: mpsc::Sender<bool>,
}

fn launch_control_thread() -> anyhow::Result<ThreadManager> {
    info!("start control thread 🚀");

    debug!("start bind broadcast socket 0.0.0.0:55000");
    let broadcast_socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 55000)))?;
    debug!("success bind broadcast socket ✅");

    info!("start receiving broadcast on 0.0.0.0:55000...");
    let (_, src) = broadcast_socket.recv_from(&mut Vec::new())?;
    if log_enabled!(log::Level::Info) {
        info!("received broadcast from {:?}", src);
    }

    if log_enabled!(log::Level::Debug) {
        debug!("start bind control socket {:?}:{:?}", USER_IP, CMD_PORT);
    }
    let control_socket = UdpSocket::bind(SocketAddr::from((USER_IP, CMD_PORT)))?;
    debug!("success bind control socket ✅");

    debug!("set control socket read timeout to 2 seconds");
    control_socket.set_read_timeout(Some(Duration::from_millis(2000)))?;
    debug!("success set control socket read timeout ✅");

    let mut seq = 0u16;
    if log_enabled!(log::Level::Info) {
        info!("sending handshake request to {:?}", src);
    }
    control_socket.send_to(&ControlFrame::new(seq, HANDSHAKE_REQ).serialize()?, src)?;
    info!("success sent handshake request ✅");

    seq = seq.checked_add(1).unwrap_or_default();

    let mut buffer = [0; 1024];

    info!("trying to receive handshake response...");
    control_socket.recv_from(&mut buffer)?;
    info!("success received handshake response ✅");
    
    debug!("deserializing handshake response...");
    let (mut lidar_seq, handshake_resp): (u16, HandshakeResp) = ControlFrame::deserialize(&buffer)?;
    debug!("success deserialized handshake response ✅");
    
    if log_enabled!(log::Level::Debug) {
        debug!("received handshake response: {:?}", handshake_resp);
    }
    if handshake_resp.check_status() {
        warn!("handshake failed, received: {:?}", handshake_resp);
        return Err(anyhow::anyhow!("handshake failed"));
    }
    info!("handshake success ✅");
    let (tx, rx) = mpsc::channel();
    let handle: JoinHandle = thread::spawn(move || {
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

use env_logger::{Builder, Target};
use std::env;

fn main() -> anyhow::Result<()> {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);

    builder.init();

    let control_thread = launch_control_thread()?;
    let data_socket = UdpSocket::bind(SocketAddr::from((USER_IP, DATA_PORT)))?;
    Ok(())
}
