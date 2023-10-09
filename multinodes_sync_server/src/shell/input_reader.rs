
use std::io::{stdin, stdout, Write};

pub enum InputResult {
    // Interrupted,
    // EOF,
    CMD(String),
}


pub struct InputReader;

impl InputReader  {

    pub fn read(&mut self, prompt: &str) -> InputResult {

        print!("{}", prompt);
        stdout().flush().unwrap();

        // Read text
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Error: Could not read a line");

        InputResult::CMD(buffer)
    }
}