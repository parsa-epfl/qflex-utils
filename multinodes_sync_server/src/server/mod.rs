use tokio::task;
use tokio::net::{UnixListener, UnixStream};
use std::fs;
use std::io::ErrorKind;
use std::thread::sleep;
use std::time::Duration;

enum SyncMessage {
    HELLO,
}

pub struct SyncServer {
    // Store server socket/id or else in somthing

    budget: u32,
    nb_of_slave: u8,
    socket_path: &'static str,
    counter: u64,

}

impl SyncServer {
    pub fn new(socket_path: String, budget: u32) -> Self {
        Self {
            budget,
            nb_of_slave: 0,
            socket_path: socket_path.trim(),
            counter: 0
        }
    }

    fn spawn(&self) {
        // Spawn X treads for the X salves
        for slave_index in 0..self.nb_of_slave {
            tokio::spawn(async move {

                let ret =
                    match ret {

                    }

            });
        }
    }

    pub async fn main(&self) {

        let mut sleep_time : u64 = 0;

        self.spawn();

        for slave_index in 0..self.nb_of_slave {
            let s2m_path = format!("{}/s2m_{:0>2}", self.socket_path, slave_index);
            let _ = fs::remove_file(&s2m_path);
        }

        for slave_index in 0..self.nb_of_slave {
            let s2m_path = format!("{}/s2m_{:0>2}", self.socket_path, slave_index);
            let mut connection_status = UnixStream::connect(&s2m_path).await;
            

            while let Err(e) = connection_status {
                match e.kind() {
                    ErrorKind::NotFound => {
                        println!("Socket not found, retrying...in {}", sleep_time);
                        sleep(Duration::from_secs(sleep_time)).await;
                    }
                    other => todo!(),

                }

            }
        }



    }
}