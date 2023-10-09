// @inspired from https://gitlab.com/john_t/shellfish


use std::thread;

mod shell;
mod server;

use shell::Shell;
use shell::command::Command;
use server::SyncServer;


fn main() {

    let mut shell = Shell::new("(SS) > ".to_string());

    shell.cmds.insert(
        "test", Command::new("Test smth".to_string(), test)
    );


    // Create a main Server Threads
    // let server = SyncServer::new(String::from("8080"));
    // let _handler = thread::spawn(move || {
    //    let _ = server.main();
    // });

    let _ = SyncServer::new(
        "/var/run/ss".to_string(), 10000)
        .main();
    // shell.run();
}



fn test()
{
    println!("Greetings my good friend.");

}