// @inspired from https://gitlab.com/john_t/shellfish
//
// mod shell;
//
// use shell_loop::command::Command;
// use crate::shell_loop::Shell;


mod shell;

use shell::Shell;
use shell::command::Command;

fn main() {

    let mut shell = Shell::new("(SS) > ".to_string());

    shell.cmds.insert(
        "test", Command::new("Test smth".to_string(), test)
    );


    shell.run();
}



fn test()
{
    println!("Greetings my good friend.");

}