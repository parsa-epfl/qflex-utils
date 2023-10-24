use bincode::Encode;

#[derive(Debug, Clone)]
pub enum ChannelMessage {
    Stop,
    Start,
    Snap(String),
    NoFence,
    Fence(u32),
    Terminate
}

#[derive(Encode, Debug, Clone, PartialEq)]
pub enum SyncMessageType {
    Null,
    Stop,
    Start,
    Snap(String),
    NoFence,
    Fence(u32),
    Terminate,
}