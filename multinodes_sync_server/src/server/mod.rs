use std::path::{Path, PathBuf};
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs;
use std::io::{Read, Write};
use std::thread::{JoinHandle, Thread};

use bincode::config::LittleEndian;
use bincode::{Encode,
    config::{
        Configuration,
        BigEndian,
        self,
        Fixint,
    }
};

use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex, Condvar, Barrier};


const MAX_FRAME_BYTE_SIZE: usize = 256;
const SERIALIZE_CONFIG: Configuration<LittleEndian, Fixint> =  config::standard().with_little_endian().with_fixed_int_encoding();


#[derive(Encode, PartialEq, Debug)]
enum SyncMessageType {
    Null,
    Stop,
    Start,
    Snap(String),
    NoFence,
    Fence(u32),
    Terminate,
    Done,
}

#[derive(Clone)]
pub struct SyncServer {
    budget: u32,
    nb_of_slave: usize,
    socket_path: PathBuf,

    // socket_ready_lock: Arc<(Mutex<u8>, Condvar)>,
    // socket_barrier: Arc<Barrier>,

}

impl SyncServer {

    pub fn new(socket_path: String, budget: u32, nb_of_slave: usize) -> Self
    {


        // TODO Implement CondVar stuff

        Self {
            budget,
            nb_of_slave,
            // socket_ready_lock: Arc::new((Mutex::new(0), Condvar::new())),
            // socket_barrier: Arc::new(Barrier::new(nb_of_slave)),
            socket_path: PathBuf::from(socket_path),
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

        println!("Server started, waiting for clients at {}", self.socket_path.display());


        let mut thread_handles: Vec<JoinHandle<_>> = Vec::with_capacity(self.nb_of_slave);

        let  socket_barrier = Arc::new(Barrier::new(self.nb_of_slave));

        for stream in listener.incoming() {

            let local_thread_ready_lock = Arc::clone(&socket_barrier);
            let budget = self.budget;

            match stream
            {
                Ok(stream) => thread_handles.push(
                    thread::spawn(
                        move || { SyncServer::handle_client(
                            stream,
                            local_thread_ready_lock,
                            budget); }
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

    fn handle_client(
        mut stream: UnixStream,
        ready_lock: Arc<Barrier>,
        starting_budget: u32)
    {
        // Send Stop anyway, then set budget
        SyncServer::send_message(&mut stream, SyncMessageType::Stop);
        SyncServer::send_message(&mut stream, SyncMessageType::Fence(starting_budget));

        let mut done_msg: String = String::with_capacity(8);

        loop {

            println!("[{:?}] Starting to wait", stream);
            ready_lock.wait();

            SyncServer::send_message(&mut stream, SyncMessageType::Start);

            println!("[{:?}] Starting to wait for DONE", stream);
            stream.read_to_string(&mut done_msg).unwrap();
            println!("Received: {}", done_msg);


        }




            // match decoded {
            //     SyncMessageType::Done => println!("Yep that's it"),
            //     _ => println!("NOP")
            // }




        // loop {
        //     println!("Entering loop, ... now wait");
        //     ready_lock.wait();
        //     println!("After loop");
        //     // Wait channel message
        //     // receive.stuff()
        // }



        // Implement STATE MACHINE
        // Send STOP
        // Wait for channel message from SyncMaster
            // On receive send write START/STOP/SNAP

    }

    fn send_message(stream: &mut UnixStream, message: SyncMessageType)
    {

        //? @see https://github.com/bincode-org/bincode/blob/trunk/docs/spec.md
        let mut encoded_msg: [u8; MAX_FRAME_BYTE_SIZE] = [0; MAX_FRAME_BYTE_SIZE];
        bincode::encode_into_slice(&message, &mut encoded_msg, SERIALIZE_CONFIG).unwrap();
        match stream.write(&encoded_msg) {
            Err(e) => println!("Something went somewhat wrong, {}", e.kind()),
            Ok(_) => println!("Send message {:?}", message),
        };

    }



}