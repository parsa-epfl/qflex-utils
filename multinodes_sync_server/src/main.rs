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





    let _ = SyncServer::new("/run/ss".to_string(), 10000, 2).listen();
    // match server.run() {
    //     Ok(_) => (),
    //     Err(e) => eprintln!("Server error: {}", e),
    // }

    // shell.run();
}



fn test()
{
    println!("Greetings my good friend.");

}