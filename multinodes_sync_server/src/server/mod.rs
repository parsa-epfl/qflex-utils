use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};
use std::time;

use std::sync::{Arc, Barrier, Condvar, Mutex};

mod socket_handler;
use socket_handler::SocketHandler;

mod circular_buffer;
use circular_buffer::CircularBuffer;

pub mod message;
use message::{ChannelMessage, SyncMessageType};

pub struct SyncServer {
    nb_of_slave:    u16,
    budget:         u32,
    socket_path:    PathBuf,
    socket_barrier: Arc<Barrier>,
    thread_handles: Vec<JoinHandle<()>>,
    stop_lock:      Arc<(Mutex<bool>, Condvar)>,
    ring_buffer:    Arc<Mutex<CircularBuffer<SyncMessageType>>>,
}

impl SyncServer {
    pub fn new(
        nb_of_slave: u16,
        budget: u32,
        socket_path: String,
    ) -> Self {

        Self {
            budget,
            nb_of_slave,

            socket_path:    PathBuf::from(socket_path),
            ring_buffer:    Arc::new(
                            Mutex::new(
                                CircularBuffer::new(
                                    256,
                                    nb_of_slave))),
            stop_lock:      Arc::new((Mutex::new(true), Condvar::new())),
            socket_barrier: Arc::new(Barrier::new(nb_of_slave.into())),
            thread_handles: Vec::with_capacity(usize::from(nb_of_slave) + 1)
        }
    }

    pub fn listen(mut self, rx: Option<Receiver<ChannelMessage>>) -> std::io::Result<()> {

        // Delete socket file if already exist
        if self.socket_path.exists() { fs::remove_file(&self.socket_path).unwrap(); }

        // Bind an incoming connection listener to a Unix file socket
        let listener = match UnixListener::bind(&self.socket_path) {
            Ok(socket   )=> socket,
            Err(_)       => panic!("failed to bind socket {}", self.socket_path.display()),
        };

        info!("Server started, waiting for clients on {}",self.socket_path.display());


        let with_buffer = rx.is_some();

        if rx.is_some() {

            let receiver_th_lock    = Arc::clone(&self.stop_lock);
            let receiver_th_buffer  = Arc::clone(&mut self.ring_buffer);

            self.thread_handles.push(
                thread::spawn(move || {
                    SyncServer::listen_2_shell(
                       rx.unwrap(),
                        receiver_th_lock,
                        receiver_th_buffer,
                    );
                })
            );

        }


        for stream in listener.incoming() {
            self.accept_incoming_connection(stream, with_buffer);
        }

        for handle in self.thread_handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    fn push_payload2buffer(
        message: SyncMessageType,
        ring_buffer: &Arc<Mutex<CircularBuffer<SyncMessageType>>>,
    ) {

        let mut lock = ring_buffer.lock().unwrap();

        while let Err(e) = lock.push(message.clone())
        {
            warn!("{e}");
            thread::sleep(time::Duration::from_secs(1));
        }

        info!("Pushed message {message:?} to ring buffer")

    }

    fn listen_2_shell(
        rx: Receiver<ChannelMessage>,
        stop_lock: Arc<(Mutex<bool>, Condvar)>,
        ring_buffer: Arc<Mutex<CircularBuffer<SyncMessageType>>>,

    ) {
        info!("Starting channel listener threads");

        loop {
            let mess_type = rx.recv().expect("MPSC Communication channel broke");

            info!("Incomming message from SHELL => {:?}", mess_type);

            match mess_type
            {
                ChannelMessage::Stop => {
                    let (mutex, _) = &*stop_lock;
                    let mut is_stopped = mutex.lock().unwrap();
                    *is_stopped = true;
                },
                ChannelMessage::Start => {
                    let (mutex, cvar) = &*stop_lock;
                    let mut is_stopped = mutex.lock().unwrap();
                    *is_stopped = false;
                    cvar.notify_all();
                }

                ChannelMessage::Fence(budget)   => SyncServer::push_payload2buffer(SyncMessageType::Fence(budget), &ring_buffer),
                ChannelMessage::NoFence         => SyncServer::push_payload2buffer(SyncMessageType::NoFence, &ring_buffer),
                ChannelMessage::Snap(dirname)   => SyncServer::push_payload2buffer(SyncMessageType::Snap(dirname), &ring_buffer),
                ChannelMessage::Terminate       => SyncServer::push_payload2buffer(SyncMessageType::Terminate, &ring_buffer)
            }

        }

    }

    fn accept_incoming_connection(&mut self, unixsocket: Result<UnixStream, std::io::Error  >, with_buffer: bool)
    {

        let socket_stop_lock_ptr = Arc::clone(&self.stop_lock);
        let socket_buffer_ptr    = Arc::clone(&self.ring_buffer);
        let socket_barrier_ptr   = Arc::clone(&self.socket_barrier);

        match unixsocket {
            Ok(stream) => {

                self.thread_handles.push(
                    thread::spawn(
                        move || {
                            SocketHandler::new(
                                stream,
                                socket_barrier_ptr,
                                socket_stop_lock_ptr,
                                socket_buffer_ptr,
                            ).handle(with_buffer);
                        }
                    )
                )

            }

            Err(err) => error!("Ouch, problem in incoming request, ({err})"),
        }
    }
}
