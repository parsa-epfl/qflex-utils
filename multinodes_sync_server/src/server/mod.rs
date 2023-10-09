use std::path::{Path, PathBuf};
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs;

enum SyncMessage {
    HELLO,
}

pub struct SyncServer {
    // Store server socket/id or else in somthing

    budget: u32,
    nb_of_slave: u8,
    socket_path: PathBuf,
    counter: u64,

}

impl SyncServer {
    pub fn new(socket_path: String, budget: u32) -> Self {
        Self {
            budget,
            nb_of_slave: 0,
            socket_path: PathBuf::from(socket_path),
            counter: 0
        }
    }


    // fn spawn(&self) {}
    pub fn main(&self) -> std::io::Result<()> {
        // let socket = match UnixStream::connect(self.socket_path.to_owned())
        // {
        //     Ok(s) => s,
        //     Err(e) =>
        //     {
        //         panic!("Couldn't connect: {}", e.kind());
        //     },
        // };

        if self.socket_path.exists() {
            fs::remove_file(&self.socket_path).unwrap();
        }


        let stream = match UnixListener::bind(&self.socket_path) {
            Err(_) => panic!("failed to bind socket"),
            Ok(stream) => stream,
        };

        println!("Server started, waiting for clients");

        // Iterate over clients, blocks if no client available
        for mut client in stream.incoming() {
                println!("Client said: {}", ???);
        }

        // let listener = UnixListener::bind(&self.socket_path)?;
        // loop {
        //     match listener.accept() {
        //         Ok((socket, addr)) => println!("Got a client: {addr:?}"),
        //         Err(e) => println!("accept function failed: {e:?}"),
        //     }
        // }

        Ok(())
    }
}