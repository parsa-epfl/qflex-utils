// @inspired from https://gitlab.com/john_t/shellfish


use std::thread;
use std::sync::mpsc::{self, SyncSender, Receiver};

mod shell;
mod server;


use server::message::SyncMessageType;


use shell::Shell;
use shell::command::Command;
use server::SyncServer;

#[macro_use] extern crate log;
use simplelog::*;
use std::fs::File;


fn main() {

    let log_config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_thread_mode(ThreadLogMode::Both)
        .build();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, log_config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, log_config.clone(), File::create("./output.log").unwrap()),
        ]
    ).unwrap();

    // create communication channel, @see: https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html
    let (tx, rx): (SyncSender<SyncMessageType>, Receiver<SyncMessageType>) = mpsc::sync_channel(0);

    let mut shell = Shell::new("(SS) > ".to_string(), tx);

    shell.cmds.insert(
        "stop", Command::new("Stop !!!".to_string(), stop)
    );


    let mut server = SyncServer::new(
        "/var/run/ss".to_string(), 
        10000, 
        1,
    );

    let handle = thread::spawn(move || {
        server.listen(rx).unwrap();
    });

    shell.run();

    handle.join().expect("Fail to join SS thread");
}



fn stop(tx: &SyncSender<SyncMessageType>)
{
    tx.send(SyncMessageType::Stop).unwrap();
}