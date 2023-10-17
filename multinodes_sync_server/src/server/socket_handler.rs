use std::{
    os::unix::net::UnixStream, 
    sync::{Arc, Barrier, Mutex}, 
    io::{Read, Write},
};

use bincode::config::{
    self,
    Configuration,
    LittleEndian,
    Fixint,
};


use super::{message::SyncMessageType, circular_buffer::CircularBuffer};

const QFLEX_MESSAGE_SIZE: usize         = 8;
const MAX_FRAME_BYTE_SIZE: usize        = 256;
const QFLEX_END_OF_BUDGET_MESSAGE: &str = "DONE";

const SERIALIZE_CONFIG: Configuration<LittleEndian, Fixint> =  
    config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();




pub struct SocketHandler{}

impl SocketHandler
{

    pub fn handle(
        mut stream: UnixStream,
        ready_lock: Arc<Barrier>,
        starting_budget: u32,
        buffer: Arc<Mutex<CircularBuffer<SyncMessageType>>>)
    {
        // Send Stop anyway, then set budget
        SocketHandler::send_message(&mut stream, SyncMessageType::Stop);
        SocketHandler::send_message(&mut stream, SyncMessageType::Fence(starting_budget));

        let mut from_socket_buffer: [u8; QFLEX_MESSAGE_SIZE] = [0; QFLEX_MESSAGE_SIZE];
        let mut str_from_buffer: &str;

        let mut read_ptr: usize = 0;

        loop {
            debug!("Consuming buffer");
            SocketHandler::consuming_buffer(&buffer, &mut read_ptr, &mut stream);


            debug!("Starting to wait");
            ready_lock.wait();
            
            debug!("Sending START");
            SocketHandler::send_message(&mut stream, SyncMessageType::Start);

            debug!("Starting to wait for {QFLEX_END_OF_BUDGET_MESSAGE}");
            loop {
                // Empty buffer first
                from_socket_buffer.fill(0);

                match stream.read(&mut from_socket_buffer[..])
                {
                    Ok(_) =>  {
                        if *from_socket_buffer.first().unwrap() == 0 {continue;}
                        debug!("Received: {:?}", from_socket_buffer);
                    },
                    Err(e) => error!("Could not read from stream => {}", e.kind()), 
                }
                
                str_from_buffer = std::str::from_utf8(&from_socket_buffer).unwrap();
                
                if str_from_buffer.contains(QFLEX_END_OF_BUDGET_MESSAGE)
                {
                    debug!("Got {QFLEX_END_OF_BUDGET_MESSAGE}, goes to wait");
                    break;
                }
            }
        }
    }

    fn send_message(
        stream: &mut UnixStream, 
        message: SyncMessageType)
    {

        //? @see https://github.com/bincode-org/bincode/blob/trunk/docs/spec.md
        let mut encoded_msg: [u8; MAX_FRAME_BYTE_SIZE] = [0; MAX_FRAME_BYTE_SIZE];
        bincode::encode_into_slice(&message, &mut encoded_msg, SERIALIZE_CONFIG).unwrap();
        match stream.write(&encoded_msg) {
            Ok(_) => info!("Send message {:?}", message),
            Err(e) => error!("Something went somewhat wrong => '{}'", e.kind()),
        };

    }

    fn consuming_buffer(
        buffer: &Arc<Mutex<CircularBuffer<SyncMessageType>>>, 
        read_ptr :&mut usize, 
        stream: &mut UnixStream)
    {
        let mut is_stopped: bool = false;

        loop {
            let mut lock = buffer.lock().unwrap();

            let message_type = lock.read(read_ptr);

            match message_type {
                Err(e) => {

                    debug!("{}", e); 
                    
                    if !is_stopped {break};

                },
                Ok(value) => {

                    info!("Got value {:?}", *value);

                    is_stopped = if *value == SyncMessageType::Stop {true} else {is_stopped};
                    is_stopped = if *value == SyncMessageType::Start {false} else {is_stopped};


                    SocketHandler::send_message(stream, value.clone());
                
                },
            }
        }
    }
}  