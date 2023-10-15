use std::{
    os::unix::net::UnixStream, 
    sync::{Arc, Barrier}, 
    io::{Read, Write}
};

use bincode::config::{
    self,
    Configuration,
    LittleEndian,
    Fixint,
};


use thread_id;
use super::message::SyncMessageType;

const MAX_FRAME_BYTE_SIZE: usize = 256;
const SERIALIZE_CONFIG: Configuration<LittleEndian, Fixint> =  
    config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();


#[macro_export]
macro_rules! th_println {
    ( $($arg:tt)* ) => {
        {   
            println!("[0x{:x}] {}", thread_id::get(), format_args!($($arg)*))
        }
    };
}


pub struct SocketHandler
{

}

impl SocketHandler
{



    pub fn handle(
        mut stream: UnixStream,
        ready_lock: Arc<Barrier>,
        starting_budget: u32)
    {
        // Send Stop anyway, then set budget
        SocketHandler::send_message(&mut stream, SyncMessageType::Stop);
        SocketHandler::send_message(&mut stream, SyncMessageType::Fence(starting_budget));

        let mut from_socket_buffer: [u8; 8] = [0; 8];
        let mut str_from_buffer: &str;

        loop {

            th_println!("Starting to wait");
            ready_lock.wait();
            
            th_println!("Sending START");
            SocketHandler::send_message(&mut stream, SyncMessageType::Start);

            th_println!("Starting to wait for DONE");
            loop {
                // Empty buffer first
                from_socket_buffer.fill(0);

                match stream.read(&mut from_socket_buffer)
                {
                    Ok(_) =>  th_println!("Received: {:?}", from_socket_buffer),
                    Err(e) => panic!("Could not read from stream => {}", e.kind()), 
                }
                
                str_from_buffer = std::str::from_utf8(&from_socket_buffer).unwrap();
                
                if str_from_buffer.contains("DONE")
                {
                    th_println!("Got DONE, goes to wait");
                    break;
                }

            }
        }
    }

    fn send_message(stream: &mut UnixStream, message: SyncMessageType)
    {

        //? @see https://github.com/bincode-org/bincode/blob/trunk/docs/spec.md
        let mut encoded_msg: [u8; MAX_FRAME_BYTE_SIZE] = [0; MAX_FRAME_BYTE_SIZE];
        bincode::encode_into_slice(&message, &mut encoded_msg, SERIALIZE_CONFIG).unwrap();
        match stream.write(&encoded_msg) {
            Ok(_) => th_println!("Send message {:?}", message),
            Err(e) => th_println!("Something went somewhat wrong => '{}'", e.kind()),
        };

    }



}