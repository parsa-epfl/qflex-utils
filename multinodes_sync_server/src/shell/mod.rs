
use std::collections::HashMap;

pub mod command;
mod input_reader;

use command::Command;
use input_reader::{InputReader, InputResult};



pub struct Shell<'a> {
    pub prompt: String,
    pub cmds: HashMap<&'a str, Command>,
    pub input_reader: InputReader,
}

impl<'a> Shell<'a> {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            cmds: HashMap::new(),
            input_reader: InputReader
        }
    }

    pub fn run(&mut self)
    {
        '_shell: loop {
            let unprocessed_line =
                match self.input_reader.read(self.prompt.as_str()) {
                    InputResult::CMD(line) => line,
                    // _other => continue '_shell,
                };

            // TODO: Escape char, stuff, and other sequences

            let command = self.cmds.get(&*unprocessed_line.trim());

            match command {
                Some(c) => { (c.command)() }
                None => { panic!("Something unusual appened: cmd={}", unprocessed_line.trim()) }
            }
        }
    }
}