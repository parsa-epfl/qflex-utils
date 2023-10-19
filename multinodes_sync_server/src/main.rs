// @inspired from https://gitlab.com/john_t/shellfish


use std::thread;
use getopts::Options;
use std::env;

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

  let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optopt("f", "socket", "Socket file", "FILE");
    opts.optopt("n", "num_slaves", "Number of slaves", "NUMBER");
    opts.optopt("b", "budget", "Budget in ms for synchronization", "NUMBER");
    
    // Extract all arguments
    let arguments = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_e) => {
            //println!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    if arguments.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return;
    }
    
    let total_slaves = arguments.opt_str("n").unwrap().parse().unwrap();
    let budget: u32 = arguments.opt_str("b").unwrap().parse().unwrap();
    let socket_path = arguments.opt_str("f").unwrap();
    
    println!("Starting Master server with {} budget.", budget);

    let _ = SyncServer::new(socket_path, budget, total_slaves).listen();

}



fn test()
{
    println!("Greetings my good friend.");

}