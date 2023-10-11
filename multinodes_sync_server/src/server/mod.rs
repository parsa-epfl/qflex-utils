use std::collections::hash_set::Union;
use std::path::{Path, PathBuf};
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs;
use std::io::Write;
use std::thread::{JoinHandle, Thread};

use bincode::{config, Encode, };
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex, Condvar};


const MAX_FILE_NAME_BYTES_SIZE: usize = 255;
//
// const STOP_ID   : usize = 1;
// const START_ID  : usize = 2;
// const SNAP_ID   : usize = 3;
// const NOFENCE_ID: usize = 4;
// const FENCE_ID  : usize = 5;
// const TERMINATE : usize = 6;

#[derive(Encode, PartialEq, Debug)]
enum SyncMessageType {
    Stop(),
    Start,
    Snap(String),
    NoFence,
    Fence(u64),
    Terminate,
}

// #[repr(C)]
// union SyncMessagePayload {
//     filename: Vec<u8>,
//     budget: u64,
// }
//
// impl SyncMessagePayload {}

// struct SyncMessage {
//     id: u8,
//     payload: Option<SyncMessagePayload>
// }

// impl SyncMessage
// {
//     pub fn new(mess_type: SyncMessageType) -> Self {
//
//         let mut id: u8;
//         let mut payload: Option<SyncMessagePayload> = None;
//
//
//         match mess_type {
//             SyncMessageType::Stop => id = 1,
//             SyncMessageType::Start => id = 2,
//             SyncMessageType::Snap(name) =>  {
//                 id = 3;
//                 payload = Some(SyncMessagePayload { filename: > });
//             },
//             SyncMessageType::NoFence => id = 4,
//             SyncMessageType::Fence(budget) => {
//                 id = 5;
//                 payload = Some(SyncMessagePayload { budget });
//             },
//             SyncMessageType::Terminate => id = 6,
//         };
//
//
//         Self {
//             id,
//             payload,
//         }
//     }
// }

pub struct SyncServer {
    budget: u32,
    nb_of_slave: u8,
    socket_path: PathBuf,

    // socket_ready_lock: Arc<()>,

    // communication_channels:
}

impl SyncServer {

    pub fn new(socket_path: String, budget: u32, nb_of_slave: u8) -> Self
    {


        // TODO Implement CondVar stuff

        Self {
            budget,
            nb_of_slave,
            //socket_ready_lock: Arc::new((Mutex::new(0), Condvar::new())),
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


        let mut thread_handles: Vec<JoinHandle<_>> = vec![];


        for stream in listener.incoming() {
            match stream
            {
                Ok(stream) => thread_handles.push(
                    thread::spawn(
                        move || { SyncServer::handle_client(stream); }
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

    fn handle_client(mut stream: UnixStream)
    {
        // Send Stop anyway
        SyncServer::send_message(&mut stream, SyncMessageType::Fence(20000));



        loop {
            // Wait channel message
        }



        // Implement STATE MACHINE
        // Send STOP
        // Wait for channel message from SyncMaster
            // On receive send write START/STOP/SNAP

    }

    fn send_message(stream: &mut UnixStream, message: SyncMessageType)
    {


        // Encoding an unsigned integer v (of any type excepting u8) works as follows:
        //
        //     If u < 251, encode it as a single byte with that value.
        //     If 251 <= u < 2**16, encode it as a literal byte 251, followed by a u16 with value u.
        //     If 2**16 <= u < 2**32, encode it as a literal byte 252, followed by a u32 with value u.
        //     If 2**32 <= u < 2**64, encode it as a literal byte 253, followed by a u64 with value u.
        //     If 2**64 <= u < 2**128, encode it as a literal byte 254, followed by a u128 with value u.

        let c = config::standard().with_big_endian().with_variable_int_encoding();

        let mut encoded_msg: [u8; MAX_FILE_NAME_BYTES_SIZE] = [0; MAX_FILE_NAME_BYTES_SIZE];

        bincode::encode_into_slice(&message, &mut encoded_msg, c).unwrap();
        stream.write(&encoded_msg).unwrap();

    }



}