use bincode::Encode;

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