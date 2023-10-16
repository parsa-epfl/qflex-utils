// @inspired from https://gitlab.com/john_t/shellfish


use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

mod shell;
mod server;


use server::message::SyncMessageType;


use shell::Shell;
use shell::command::Command;
use server::SyncServer;


fn main() {
    // create communication channel
    let (tx, rx): (Sender<SyncMessageType>, Receiver<SyncMessageType>) = mpsc::channel();

    let mut shell = Shell::new("(SS) > ".to_string(), tx);

    shell.cmds.insert(
        "stop", Command::new("Stop !!!".to_string(), stop)
    );


    let mut server = SyncServer::new(
        "/var/run/ss".to_string(), 
        10000, 
        2,
    );

    let handle = thread::spawn(move || {
        server.listen(rx).unwrap();
    });

    // shell.run();

    handle.join().expect("Fail to join SS thread");
}



fn stop(tx: &Sender<SyncMessageType>)
{
    tx.send(SyncMessageType::Stop).unwrap();
}