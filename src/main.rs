use env_logger::{Builder, Target};
use livox_lidar_rs::network_frame::cfg::{CMD_PORT, DATA_PORT, USER_IP};
use livox_lidar_rs::network_frame::control_frame::{
    deserialize_resp, CheckStatus, Cmd, CommonResp, ControlFrame, DisconnectReq, GetCmd, Len,
    SampleCtrlReq, DISCONNECT_REQ, HANDSHAKE_REQ, HEARTBEAT_REQ, SAMPLE_START_REQ,
};
use log::{debug, info, log_enabled, warn};
use serde::Serialize;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

type AnyhowHandle = thread::JoinHandle<anyhow::Result<()>>;

type TransmitMap = HashMap<Cmd, mpsc::Sender<Vec<u8>>>;
type ReceiveMap = HashMap<Cmd, mpsc::Receiver<Vec<u8>>>;

fn heartbeat_daemon(
    command_emitter: Arc<CommandEmitter>,
) -> anyhow::Result<(AnyhowHandle, mpsc::Sender<()>)> {
    debug!("heartbeat thread started ✅");

    let (tx, rx) = mpsc::channel();

    let time_to_live = Duration::from_millis(1000);

    // launch heartbeat daemon, in which send heartbeat request every 1 second
    let handle: AnyhowHandle = thread::spawn(move || loop {
        match rx.try_recv() {
            Err(_) => {
                debug!("heartbeat daemon: no sig_term received, continue...");
                let _: CommonResp = command_emitter.execute_command(HEARTBEAT_REQ)?;
                thread::sleep(time_to_live);
            }
            Ok(_) => {
                info!("received sig_term, heartbeat daemon exiting...");
                return Ok(());
            }
        }
    });
    Ok((handle, tx))
}

struct CommandEmitter {
    control_socket: UdpSocket,
    seq_ref: Mutex<u16>,
    transmit_map: Arc<Mutex<TransmitMap>>,
    receive_map: Arc<Mutex<ReceiveMap>>,
    term_sender: mpsc::Sender<()>,
}

impl CommandEmitter {
    fn new(lidar_addr: SocketAddr, control_socket: UdpSocket) -> Self {
        if let Err(e) = control_socket.connect(lidar_addr) {
            if log_enabled!(log::Level::Error) {
                warn!("error occurred when connecting to lidar: {}", e);
            }
        }
        let (tx, rx) = mpsc::channel();
        let duplicated_control_socket = control_socket.try_clone().unwrap();

        let transmit_map: Arc<Mutex<TransmitMap>> = Arc::new(Mutex::new(HashMap::new()));
        let receive_map: Arc<Mutex<ReceiveMap>> = Arc::new(Mutex::new(HashMap::new()));
        let duplicated_transmit_map = transmit_map.clone();

        // start command response receiver, receiving all response in this thread, and sending to corresponding channel
        let _: AnyhowHandle = thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                if let Ok(_) = rx.try_recv() {
                    if log_enabled!(log::Level::Info) {
                        info!("received sig_term, command response receiver exiting...");
                    }
                    return Ok(());
                }
                debug!("command response receiver: no sig_term received, continue...");
                match control_socket.recv(&mut buffer) {
                    Ok(size) => match deserialize_resp(&buffer[..size]) {
                        Ok((_, cmd, frame)) => {
                            if log_enabled!(log::Level::Debug) {
                                debug!("received response: {:?}", cmd);
                            }

                            transmit_map
                                .lock()
                                .unwrap()
                                .get(&cmd.clone())
                                .unwrap()
                                .send(frame.to_vec())
                                .unwrap()
                        }
                        Err(e) => {
                            if log_enabled!(log::Level::Warn) {
                                warn!("error occurred when deserializing response: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        if log_enabled!(log::Level::Warn) {
                            warn!("error occurred when receiving response: {}", e);
                        }
                    }
                }
            }
        });
        Self {
            control_socket: duplicated_control_socket,
            seq_ref: Mutex::new(0),
            transmit_map: duplicated_transmit_map,
            receive_map,
            term_sender: tx,
        }
    }

    /// execute certain command and return the response
    fn execute_command<T, P>(&self, req: T) -> anyhow::Result<P>
    where
        T: Len + Serialize + GetCmd,
        P: CheckStatus + for<'de> serde::Deserialize<'de>,
    {
        self.receive_map
            .lock()
            .unwrap()
            .entry(req.cmd())
            .or_insert_with(|| {
                let (tx, rx) = mpsc::channel();
                self.transmit_map.lock().unwrap().insert(req.cmd(), tx);
                rx
            });

        let mut seq = self.seq_ref.lock().unwrap();
        self.control_socket
            .send(&ControlFrame::new(*seq, &req).serialize()?)?;

        *seq = seq.checked_add(1).unwrap_or_default();
        drop(seq);

        if log_enabled!(log::Level::Debug) {
            debug!("sent command: {:?}", req.cmd());
        }

        let mes = self
            .receive_map
            .lock()
            .unwrap()
            .get(&req.cmd())
            .unwrap()
            .recv()?;
        let resp: P = bincode::deserialize(&mes)?;
        resp.check_status()?;

        if log_enabled!(log::Level::Debug) {
            debug!("command handled successfully ✅");
        }

        Ok(resp)
    }
}

fn main() -> anyhow::Result<()> {
    Builder::from_default_env().target(Target::Stdout).init();

    info!("livox lidar driver in Rust 🚀");

    let broadcast_socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 55000)))?;
    broadcast_socket.set_read_timeout(Some(Duration::from_millis(1000)))?;
    let control_socket = UdpSocket::bind(SocketAddr::from((USER_IP, CMD_PORT)))?;
    let data_socket = UdpSocket::bind(SocketAddr::from((USER_IP, DATA_PORT)))?;
    debug!("success init sockets ✅");

    control_socket.set_read_timeout(Some(Duration::from_millis(1000)))?;
    debug!("set control socket read timeout to 1 seconds");

    debug!("start listening broadcast on 0.0.0.0:55000...");
    let (_, lidar_addr) = broadcast_socket.recv_from(&mut Vec::new())?;
    if log_enabled!(log::Level::Debug) {
        debug!("received broadcast from {:?}", lidar_addr);
    }

    let command_emitter = Arc::new(CommandEmitter::new(lidar_addr, control_socket));

    debug!("trying handshake...");
    let _: CommonResp = command_emitter.execute_command(HANDSHAKE_REQ)?;
    debug!("handshake success ✅");

    info!("success connected to lidar ✅");

    let (handle, term_sender) = heartbeat_daemon(command_emitter.clone())?;
    info!("heartbeat daemon launched ✅");

    let duplicated_command_emitter = command_emitter.clone();
    // register SIGINT handler
    ctrlc::set_handler(move || {
        info!("received SIGINT in callback, disconnecting...");
        duplicated_command_emitter
            .execute_command::<DisconnectReq, CommonResp>(DISCONNECT_REQ)
            .unwrap();
        info!("lidar disconnected ✅");
        term_sender.send(()).unwrap();
        info!("heartbeat daemon terminating...");
        duplicated_command_emitter.term_sender.send(()).unwrap()
    })?;

    command_emitter.execute_command::<SampleCtrlReq, CommonResp>(SAMPLE_START_REQ)?;
    info!("success start sampling ✅");

    handle.join().unwrap()?;
    Ok(())
}
