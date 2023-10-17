
use std::fs;
use std::path::PathBuf;
use std::os::unix::net::UnixListener;
use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};
use std::time;

use std::sync::{Arc, Barrier, Mutex};

mod socket_handler;
use socket_handler::SocketHandler;

mod circular_buffer;
use circular_buffer::CircularBuffer;

pub mod message;
use message::SyncMessageType;


pub struct SyncServer {
    budget: u32,
    nb_of_slave: u16,
    socket_path: PathBuf,
    ring_buffer: Arc<Mutex<CircularBuffer<SyncMessageType>>>,

}

impl SyncServer {

    pub fn new(socket_path: String, budget: u32, nb_of_slave: u16) -> Self
    {
        Self {
            budget,
            nb_of_slave,

            socket_path: PathBuf::from(socket_path),
            ring_buffer: Arc::new(Mutex::new(CircularBuffer::new(256, nb_of_slave))),
        }
    }



    pub fn listen(&mut self, rx: Receiver<SyncMessageType>) -> std::io::Result<()>
    {
        // Delete socket file if already exist
        if self.socket_path.exists() {
            fs::remove_file(&self.socket_path).unwrap();
        }

        let listener = match UnixListener::bind(&self.socket_path) {
            Err(_) => panic!("failed to bind socket {}", self.socket_path.display()),
            Ok(socket) => socket,
        };

        info!("Server started, waiting for clients on {}", self.socket_path.display());

        let mut thread_handles: Vec<JoinHandle<_>> = Vec::with_capacity(self.nb_of_slave.into());
        let  socket_barrier = Arc::new(Barrier::new(self.nb_of_slave.into()));
        let channel_ring =  Arc::clone(&mut self.ring_buffer);

        let _rx = thread::spawn(move || {
            SyncServer::listen_2_shell(rx, channel_ring);
        });


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
                Err(err) => error!("Ouch, problem in incoming request, ({err})"),
            }
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    fn listen_2_shell(rx: Receiver<SyncMessageType>, ring: Arc<Mutex<CircularBuffer<SyncMessageType>>>)
    {

        info!("Starting channel listener threads");

        let mess_type = rx.recv().expect("MPSC Communication channel broke");

        info!("Incomming message from SHELL => {:?}", mess_type);

        {
            let mut lock = ring.lock().unwrap();
            while let Err(e) = lock.push(mess_type.clone()) 
            {
                warn!("{e}");
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    }

}