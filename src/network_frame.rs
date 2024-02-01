/// enumeration of command types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CmdType {
    Cmd,
    Ack,
    Msg,
}

pub mod control_frame;
pub mod data_frame;