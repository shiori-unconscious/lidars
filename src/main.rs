use anyhow::anyhow;
use env_logger::{Builder, Target};
use livox_lidar_rs::network_frame::cfg::{CMD_PORT, DATA_PORT, USER_IP};
use livox_lidar_rs::network_frame::control_frame::{
    CheckStatus, CommonResp, ControlFrame, DisconnectReq, HandshakeReq, Len, SampleCtrlReq,
    DISCONNECT_REQ, HANDSHAKE_REQ, HEARTBEAT_REQ, SAMPLE_START_REQ,
};
use log::{debug, error, info, log_enabled};
use serde::Serialize;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

type JoinHandle = thread::JoinHandle<anyhow::Result<()>>;

fn connect(
    broadcast_socket: UdpSocket,
    control_socket: UdpSocket,
    seq_ref: Arc<Mutex<u16>>,
) -> anyhow::Result<SocketAddr> {
    debug!("start listening broadcast on 0.0.0.0:55000...");
    error!("123");
    let (_, lidar_addr) = broadcast_socket.recv_from(&mut Vec::new())?;
    if log_enabled!(log::Level::Debug) {
        debug!("received broadcast from {:?}", lidar_addr);
    }

    debug!("trying handshake...");
    send_command::<HandshakeReq, CommonResp>(
        HANDSHAKE_REQ,
        lidar_addr,
        control_socket,
        Arc::clone(&seq_ref),
    )?;
    debug!("handshake success ✅");

    Ok(lidar_addr)
}

fn heartbeat_daemon(
    lidar_addr: SocketAddr,
    control_socket: UdpSocket,
    seq_ref: Arc<Mutex<u16>>,
) -> anyhow::Result<(JoinHandle, Sender<bool>)> {
    debug!("heartbeat thread started ✅");

    let (tx, rx) = mpsc::channel();
    let mut buffer = [0; 32];

    let time_to_live = Duration::from_millis(1000);
    control_socket.set_read_timeout(Some(time_to_live))?;

    let handle: JoinHandle = thread::spawn(move || loop {
        match rx.try_recv() {
            Err(_) => {
                debug!("no sig_term received, continue...");
                let mut seq = seq_ref.lock().unwrap();
                control_socket.send_to(
                    &ControlFrame::new(*seq, HEARTBEAT_REQ).serialize()?,
                    lidar_addr,
                )?;
                *seq = seq.checked_add(1).unwrap_or_default();
                drop(seq);
                debug!("heartbeat sent ✅");
                thread::sleep(time_to_live);
                control_socket
                    .recv_from(&mut buffer)
                    .map_err(|_| anyhow!("heartbeat timeout ❌"))?;
                debug!("heartbeat received ✅");
            }
            Ok(_) => {
                info!("received sig_term, control thread exiting...");
                return Ok(());
            }
        }
    });
    info!("heartbeat daemon launched ✅");
    Ok((handle, tx))
}

fn send_command<T, P>(
    req: T,
    lidar_addr: SocketAddr,
    control_socket: UdpSocket,
    seq_ref: Arc<Mutex<u16>>,
) -> anyhow::Result<P>
where
    T: Len + Serialize,
    P: CheckStatus + for<'de> serde::Deserialize<'de>,
{
    let mut seq = seq_ref.lock().unwrap();
    control_socket.send_to(&ControlFrame::new(*seq, req).serialize()?, lidar_addr)?;
    *seq = seq.checked_add(1).unwrap_or_default();
    drop(seq);
    let mut buffer = [0; 128];
    let (size, _) = control_socket.recv_from(&mut buffer)?;
    let (_, resp): (_, P) = ControlFrame::deserialize(&buffer[..size])?;
    resp.check_status()?;
    Ok(resp)
}

fn main() -> anyhow::Result<()> {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    info!("livox lidar driver in rust 🚀");

    let broadcast_socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 55000)))?;
    let control_socket = UdpSocket::bind(SocketAddr::from((USER_IP, CMD_PORT)))?;
    let data_socket = UdpSocket::bind(SocketAddr::from((USER_IP, DATA_PORT)))?;
    debug!("success init sockets ✅");

    control_socket.set_read_timeout(Some(Duration::from_millis(1000)))?;
    debug!("set control socket read timeout to 1 seconds");

    let seq_ref = Arc::new(Mutex::new(0));

    let lidar_addr = connect(
        broadcast_socket.try_clone()?,
        control_socket.try_clone()?,
        Arc::clone(&seq_ref),
    )?;
    info!("success connected to lidar ✅");

    let (handle, term_sender) = heartbeat_daemon(
        lidar_addr,
        control_socket.try_clone()?,
        Arc::clone(&seq_ref),
    )?;

    let term_sender1 = term_sender.clone();
    // register SIGINT handler
    ctrlc::set_handler(move || {
        info!("received SIGINT, sending sig_term to control thread...");
        term_sender1.send(true).unwrap();
    })?;

    send_command::<SampleCtrlReq, CommonResp>(
        SAMPLE_START_REQ,
        lidar_addr,
        control_socket.try_clone()?,
        Arc::clone(&seq_ref),
    )?;
    info!("success start sampling ✅");

    term_sender.send(true).unwrap();
    send_command::<DisconnectReq, CommonResp>(
        DISCONNECT_REQ,
        lidar_addr,
        control_socket.try_clone()?,
        Arc::clone(&seq_ref),
    )?;
    handle.join().unwrap()?;
    Ok(())
}
