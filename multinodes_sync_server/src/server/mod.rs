use std::fs;
use std::path::PathBuf;
use std::os::unix::net::UnixListener;
use std::thread::{self, JoinHandle};


use std::sync::{Arc, Barrier, Mutex};

mod message;
mod socket_handler;
use socket_handler::SocketHandler;

mod circular_buffer;
use circular_buffer::CircularBuffer;






pub struct SyncServer {
    budget: u32,
    nb_of_slave: usize,
    socket_path: PathBuf,
    ring_buffer: Arc<Mutex<CircularBuffer<u8>>>,

}

impl SyncServer {

    pub fn new(socket_path: String, budget: u32, nb_of_slave: usize) -> Self
    {
        Self {
            budget,
            nb_of_slave,
            // socket_ready_lock: Arc::new((Mutex::new(0), Condvar::new())),
            // socket_barrier: Arc::new(Barrier::new(nb_of_slave)),
            socket_path: PathBuf::from(socket_path),
            ring_buffer: Arc::new(Mutex::new(CircularBuffer::new(256))),
        }
    }



    pub fn listen(&mut self) -> std::io::Result<()>
    {

        // Delete socket file if already exist
        if self.socket_path.exists() {
            fs::remove_file(&self.socket_path).unwrap();
        }


        let listener = match UnixListener::bind(&self.socket_path) {
            Err(_) => panic!("failed to bind socket {}", self.socket_path.display()),
            Ok(socket) => socket,
        };

        println!("Server started, waiting for clients on {}", self.socket_path.display());


        let mut thread_handles: Vec<JoinHandle<_>> = Vec::with_capacity(self.nb_of_slave);

        let  socket_barrier = Arc::new(Barrier::new(self.nb_of_slave));

        self.ring_buffer.lock().unwrap().push(1);
        self.ring_buffer.lock().unwrap().push(5);

        for stream in listener.incoming() {

            let local_thread_ready_lock = Arc::clone(&socket_barrier);
            let local_thread_buffer = Arc::clone(&mut self.ring_buffer);
            let budget = self.budget;
            
            match stream
            {
                Ok(stream) => thread_handles.push(
                    thread::spawn(
                        move || { SocketHandler::handle(
                            stream,
                            local_thread_ready_lock,
                            budget,
                            local_thread_buffer); }
                    )
                ),
                Err(err) => eprintln!("Ouch, problem in incoming request, ({})", err),
            }
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        Ok(())
    }

}