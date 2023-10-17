

// Shell function signature, [STATE, ARGUMENTS]
// pub type CommandFn<T> = fn(&mut T, Vec<String>) -> Result<(), Box<dyn Error>>>;

use std::sync::mpsc::SyncSender;    

use crate::server::message::SyncMessageType;

pub type CommandFn = fn(&SyncSender<SyncMessageType>);

pub struct Command {
    pub command: CommandFn,
    pub help: String,
}

impl Command {

    pub fn new(help: String, command: CommandFn) -> Self {
        Self {
            command,
            help
        }
    }

}
