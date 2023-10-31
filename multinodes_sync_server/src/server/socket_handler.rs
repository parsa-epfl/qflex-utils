use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    sync::{Arc, Barrier, Condvar, Mutex},
};

use bincode::config::{self, Configuration, Fixint, LittleEndian};

use super::{circular_buffer::CircularBuffer, message::SyncMessageType};

const FRAME_BYTE_SIZE:              usize   = 256;
const QFLEX_MESSAGE_SIZE:           usize   = 8;
const QFLEX_END_OF_BUDGET_MESSAGE:  &str    = "DONE";

const SERIALIZE_CONFIG: Configuration<LittleEndian, Fixint> =
    config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();

pub struct SocketHandler
{
    stream:         UnixStream,
    ready_lock:     Arc<Barrier>,
    stop_lock:      Arc<(Mutex<bool>, Condvar)>,
    buffer_lock:    Arc<Mutex<CircularBuffer<SyncMessageType>>>,
    read_ptr:       usize
}

impl SocketHandler
{
    pub fn new(
        stream:         UnixStream,
        ready_lock:     Arc<Barrier>,
        stop_lock:      Arc<(Mutex<bool>, Condvar)>,
        buffer_lock:    Arc<Mutex<CircularBuffer<SyncMessageType>>>,
    ) -> Self {
        Self {
            stop_lock,
            stream,
            ready_lock,
            buffer_lock,
            read_ptr: 0,
        }
    }

    pub fn handle(&mut self, with_buffer: bool)
    {

        let mut from_socket_buffer: [u8; QFLEX_MESSAGE_SIZE] =
            [0; QFLEX_MESSAGE_SIZE];
        let mut str_from_buffer: &str;


        loop {
            // TODO; this kind of behaviour should be refactored into a SocketFactory 
            // or another type of creational pattern
            if with_buffer
            {
                debug!("Consuming buffer");
                self.consuming_buffer();
            }

            info!("Trying to acquire the start/stop lock");
            self.wait_for_shell_start();


            info!("Starting to wait");
            self.ready_lock.wait();

            debug!("Sending START");
            SocketHandler::send_message(&mut self.stream, SyncMessageType::Start);

            debug!("Starting to wait for {QFLEX_END_OF_BUDGET_MESSAGE}");
            loop {
                // Empty buffer first
                from_socket_buffer.fill(0);

                match self.stream.read(&mut from_socket_buffer[..]) {
                    Ok(_) => {
                        if *from_socket_buffer.first().unwrap() == 0 {
                            continue;
                        }
                        debug!("Received: {:?}", from_socket_buffer);
                    }
                    Err(e) => {
                        error!("Could not read from stream => {}", e.kind())
                    }
                }

                str_from_buffer =
                    std::str::from_utf8(&from_socket_buffer).unwrap();

                if str_from_buffer.contains(QFLEX_END_OF_BUDGET_MESSAGE) {
                    debug!("Got {QFLEX_END_OF_BUDGET_MESSAGE}, goes to wait");
                    break;
                }
            }
        }
    }

    fn send_message(stream: &mut UnixStream,  message: SyncMessageType)
    {
        //? @see https://github.com/bincode-org/bincode/blob/trunk/docs/spec.md
        let mut encoded_msg: [u8; FRAME_BYTE_SIZE] = [0; FRAME_BYTE_SIZE];

        bincode::encode_into_slice(
            &message,
            &mut encoded_msg,
            SERIALIZE_CONFIG,
        )
            .unwrap();
        match stream.write(&encoded_msg) {
            Ok(_) => info!("Send message {:?}", message),
            Err(e) => error!("Something went somewhat wrong => '{}'", e.kind()),
        };
    }

    fn consuming_buffer(&mut self)
    {

        loop {
            let mut ring_buffer = self.buffer_lock.lock().unwrap();
            let inside_lock     =  ring_buffer.read(&mut self.read_ptr);

            match inside_lock
            {
                Err(e) =>  { 
                    debug!("{}", e);
                    break;
                },
                Ok(value) => {
                    info!("Got value {:?}", value);
                    SocketHandler::send_message(&mut self.stream, value.clone());

                }
            }
        }
    }

    fn wait_for_shell_start(&self)
    {
        let (lock, cvar) = &*self.stop_lock;
        let mut is_stopped = lock.lock().unwrap();
        // As long as the value inside the `Mutex<bool>` is `false`, we wait.
        while *is_stopped
        {
            is_stopped = cvar.wait(is_stopped).unwrap();
        }
    }
}
