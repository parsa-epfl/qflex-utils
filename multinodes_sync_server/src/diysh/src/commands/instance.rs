use crate::shell::Shell;

use super::argument::EvaluatedArg;

pub struct CommandInstance<'a, T> {
    shell: &'a Shell<T>,
    arg_list: Vec<EvaluatedArg>,
    callback: fn(&Shell<T>, &Vec<EvaluatedArg>)
}

impl<'a, T> CommandInstance<'a, T> {
    pub fn new(shell: &'a Shell<T>, arg_list: Vec<EvaluatedArg>, callback: fn(&Shell<T>, &Vec<EvaluatedArg>)) -> Self {
        CommandInstance { shell, arg_list, callback }
    }

    pub fn run(&self) {
        (self.callback)(self.shell, &self.arg_list)
    }
}