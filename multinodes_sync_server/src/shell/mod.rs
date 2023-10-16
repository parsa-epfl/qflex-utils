
use std::{collections::HashMap, sync::mpsc::Sender};

pub mod command;
mod input_reader;

use command::Command;
use input_reader::{InputReader, InputResult};

use crate::server::message::SyncMessageType;



pub struct Shell<'a> {
    pub prompt: String,
    pub cmds: HashMap<&'a str, Command>,
    pub input_reader: InputReader,
    channel_2_server: Sender<SyncMessageType>
}

impl<'a> Shell<'a> {
    pub fn new(prompt: String, channel_2_server: Sender<SyncMessageType>) -> Self {
        Self {
            prompt,
            cmds: HashMap::new(),
            input_reader: InputReader,
            channel_2_server,

        }
    }

    pub fn run(&mut self)
    {
        loop {
            let unprocessed_line =
                match self.input_reader.read(self.prompt.as_str()) {
                    InputResult::CMD(line) => line,
                };

            // TODO: Escape char, stuff, and other sequences

            let command = self.cmds.get(&*unprocessed_line.trim());

            match command {
                Some(c) => { (c.command)(&self.channel_2_server) }
                None => { panic!("Something unusual appened: cmd={}", unprocessed_line.trim()) }
            }
        }
    }
}