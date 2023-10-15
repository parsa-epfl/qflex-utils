use bincode::Encode;

#[derive(Encode, Debug)]
pub enum SyncMessageType {
    Null,
    Stop,
    Start,
    Snap(String),
    NoFence,
    Fence(u32),
    Terminate,
    Done,
}