use std::thread;
use std::sync::mpsc::{self, SyncSender, Receiver};


#[macro_use] extern crate log;
use simplelog::*;
use std::fs::File;
use clap::Parser;

// No Mod because diysh is installed as a custom package
use diysh::shell::Shell;
use diysh::commands::definition::CommandDefinition;
use diysh::commands::argument::ArgType;

mod server;
use server::SyncServer;
use server::message::ChannelMessage;

fn main()
{

    
    let args = Args::parse();
    
    let log_config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_thread_mode(ThreadLogMode::Both)
        .build();
    
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, log_config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, log_config.clone(), File::create(args.logfile).unwrap()),
        ]
    ).unwrap();


    let mut server = SyncServer::new(
        args.nb_of_slave,
        args.budget,
        args.socket,
    );


    if args.shell {
        let (tx, rx): (SyncSender<ChannelMessage>, Receiver<ChannelMessage>) = mpsc::sync_channel(0);
        server.attach_channel(Option::from(rx));

        let mut shell = Shell::new(tx);
        set_shell(&mut shell);

        thread::spawn(move || {
            server.listen().unwrap();
        });


        loop { shell.read_and_run(); }
    }
    else
    {
        let handle = thread::spawn(move || { server.listen().unwrap(); });
        handle.join().unwrap();
    }


}

fn set_shell(shell: &mut diysh::shell::Shell<ChannelMessage>)
{

    shell.set_sparse(false)
        .set_prompt("(SS) > ")
        .set_log_directory("./shell_log/")
        .register_help()
        .register_history()
        .register_exit()
        
        .register_command(CommandDefinition::new("stop")
        .set_description("Send stop message")
        .set_callback(|shell, _args,| {
            shell.tx.send(ChannelMessage::Stop).unwrap();
            info!("Send 'stop' from shell");
        })
        .build())

        .register_command(CommandDefinition::new("start")
        .set_description("Send stop message")
        .set_callback(|shell, _args,| {
            shell.tx.send(ChannelMessage::Start).unwrap();
            info!("Send 'start' from shell");
        })
        .build())

        .register_command(CommandDefinition::new("snap")
        .set_description("txt:str - Make a snapshot with filename")
        .add_arg(ArgType::Str)
        .set_callback(|shell, args,| {
            shell.tx.send(ChannelMessage::Snap(args[0].get_str().unwrap())).unwrap();
            info!("Send 'snap' from shell");
        })
        .build())

        .register_command(CommandDefinition::new("fence")
        .set_description("Set guest budget")
        .add_arg(ArgType::Int)
        .set_callback(|shell, args,| {
            let budget:u32 = args[0].get_int().unwrap().abs() as u32;
            
            shell.tx.send(ChannelMessage::Fence(budget)).unwrap();
            info!("Send 'fence ({budget})' from shell");
        })
        .build())

        .register_command(CommandDefinition::new("nofence")
        .set_description("Remove budget on guests")
        .set_callback(|shell, _args,| {
            shell.tx.send(ChannelMessage::NoFence).unwrap();
            info!("Send 'nofence' from shell");
        })
        .build())

        .register_command(CommandDefinition::new("terminate")
        .set_description("text:str - Send terminate message")
        .set_callback(|shell, _args,| {
            shell.tx.send(ChannelMessage::Terminate).unwrap();
            info!("Send 'terminate' from shell");
        })
        .build());



}

#[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
#[command(name = "SyncServer")]
#[command(author = "Bryan `Brany`Perdrizat <bryan.perdrizat@epfl.ch>")]
#[command(version = "1.0")]
#[command(about = "CLI utility to manage a server that synchronise \
 quantum among running virtual instances", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t=String::from("/var/run/ss"), help="Location of the UNIX socket on the system")]
    socket: String,

    #[arg(short, long, default_value_t=2, help="Number of guest the server will wait before starting")]
    nb_of_slave: u16,

    #[arg(short, long, default_value_t=10000, help="Starting budget for the run")]
    budget: u32,

    #[arg(long, help="Spawn the server shell in foreground")]
    shell: bool,

    #[arg(long, help="Logfile", default_value_t=String::from("./output.log"))]
    logfile: String,
}


