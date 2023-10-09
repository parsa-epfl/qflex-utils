use std::path::{Path, PathBuf};
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs;
use std::thread::{JoinHandle, Thread};

enum SyncMessage {
    HELLO,
}

pub struct SyncServer {
    socket_path: PathBuf,
    budget: u32,
}

impl SyncServer {

    pub fn new(socket_path: String, budget: u32) -> Self {
        Self {
            socket_path: PathBuf::from(socket_path),
            budget,
        }
    }



    // fn spawn(&self) {}
    pub fn listen(&self) -> std::io::Result<()> {

        // Delete socket file if already exist
        if self.socket_path.exists() {
            fs::remove_file(&self.socket_path).unwrap();
        }


        let listener = match UnixListener::bind(&self.socket_path) {
            Err(_) => panic!("failed to bind socket {}", self.socket_path.display()),
            Ok(socket) => socket,
        };

        println!("Server started, waiting for clients at {}", self.socket_path.display());

        let mut connexion_threads: Vec<JoinHandle<()>> = vec![];
        for stream in listener.incoming() {
            match stream
            {
                Ok(stream) => connexion_threads.push(
                    thread::spawn(
                        move || { SyncServer::handle_client(stream); }
                    )
                ),
                Err(err) => eprintln!("Ouch, problem in incoming request, ({})", err),
            }
        }

        for child in connexion_threads
        {
            let _ = child.join();
        }

        Ok(())
    }

    fn handle_client(stream: UnixStream)
    {

    }

}