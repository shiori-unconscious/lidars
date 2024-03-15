use anyhow::anyhow;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use env_logger::{Builder, Target};
use livox_lidar_rs::network_frame::cfg::{CMD_PORT, DATA_PORT, USER_IP};
use livox_lidar_rs::network_frame::control_frame::{
    CheckStatus, ControlFrame, HandshakeResp, HANDSHAKE_REQ, HEARTBEAT_REQ,
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

    info!("start listening broadcast on 0.0.0.0:55000...");
    let (_, src) = broadcast_socket.recv_from(&mut Vec::new())?;
    if log_enabled!(log::Level::Debug) {
        debug!("received broadcast from {:?}", src);
    }

    if log_enabled!(log::Level::Debug) {
        debug!(
            "start bind control socket {:?}",
            SocketAddr::from((USER_IP, CMD_PORT))
        );
    }
    let control_socket = UdpSocket::bind(SocketAddr::from((USER_IP, CMD_PORT)))?;
    debug!("success bind control socket ✅");

    debug!("set control socket read timeout to 2 seconds");
    control_socket.set_read_timeout(Some(Duration::from_millis(2000)))?;

    let mut seq = 0u16;
    if log_enabled!(log::Level::Debug) {
        debug!("sending handshake request to {:?}", src);
    }
    control_socket.send_to(&ControlFrame::new(seq, HANDSHAKE_REQ).serialize()?, src)?;
    debug!("success sent handshake request ✅");

    seq = seq.checked_add(1).unwrap_or_default();

    let mut buffer = [0; 1024];

    debug!("trying to receive handshake response...");
    let (size, _) = control_socket.recv_from(&mut buffer)?;
    debug!("success received handshake response ✅");

    debug!("deserializing handshake response...");
    let (_, handshake_resp): (u16, HandshakeResp) =
        ControlFrame::deserialize(&buffer[..size])?;
    debug!("success deserialized handshake response ✅");

    if log_enabled!(log::Level::Debug) {
        debug!("received handshake response: {:?}", handshake_resp);
    }

    debug!("checking status code...");
    if !handshake_resp.check_status() {
        return Err(anyhow::anyhow!("handshake failed ❌, failure status code"));
    }
    debug!("handshake success ✅");
    info!("control thread connected to lidar ✅");

    let (tx, rx) = mpsc::channel();
    let handle: JoinHandle = thread::spawn(move || {
        debug!("heartbeat thread started ✅");
        control_socket.set_read_timeout(Some(Duration::from_millis(1000)))?;
        loop {
            match rx.try_recv() {
                Ok(_) => {
                    info!("received sig_term, control thread exiting...");
                    return Ok(());
                }
                Err(_) => {
                    debug!("no sig_term received, continue...");
                    control_socket
                        .send_to(&ControlFrame::new(seq, HEARTBEAT_REQ).serialize()?, src)?;
                    debug!("heartbeat sent ✅");
                    seq = seq.checked_add(1).unwrap_or_default();
                    thread::sleep(Duration::from_millis(1000));
                    control_socket
                        .recv_from(&mut buffer)
                        .map_err(|_| anyhow!("heartbeat timeout ❌"))?;
                    debug!("heartbeat received ✅");
                }
            }
        }
    });
    Ok(ThreadManager {
        handle,
        sig_term_sender: tx,
    })
}

fn main() -> anyhow::Result<()> {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    match launch_control_thread() {
        Ok(ThreadManager {
            sig_term_sender,
            handle,
        }) => {
            ctrlc::set_handler(move || {
                info!("received SIGINT, sending sig_term to control thread...");
                sig_term_sender.send(true).unwrap();
            })?;
            let data_socket = UdpSocket::bind(SocketAddr::from((USER_IP, DATA_PORT)))?;            
            handle.join().unwrap()?;
        }
        Err(e) => {
            warn!("control thread failed to start: {}", e);
        }
    }

    Ok(())
}
