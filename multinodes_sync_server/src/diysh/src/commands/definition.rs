use crate::{error::CommandError, inout::read::ArgToken, shell::Shell};

use super::{argument::{ArgType, EvaluatedArg}, instance::CommandInstance};

#[derive(Debug)]
pub struct CommandDefinition<S> {
    name: &'static str,
    arg_list: Vec<ArgType>,
    callback: fn(&Shell<S>, &Vec<EvaluatedArg>),
    description: &'static str
}

impl<S> Clone for CommandDefinition<S> {
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), arg_list: self.arg_list.clone(), callback: self.callback.clone() , description: self.description.clone()}
    }
}

impl<'a, S> CommandDefinition<S> {
    pub fn new(name: &'static str) -> Self {
        Self { name: name, arg_list: vec![], callback: |_shell, _args| {  }, description: "" }
    }

    pub fn build(&self) -> CommandDefinition<S> {
        self.clone()
    }

    pub fn add_arg(&mut self, arg_type: ArgType) -> &mut Self {
        self.arg_list.push(arg_type);

        self
    }

    pub fn set_description(&mut self, description: &'static str) -> &mut Self {
        self.description = description;

        self
    }

    pub fn set_callback(&mut self, callback: fn(&Shell<S>, &Vec<EvaluatedArg>)) -> &mut Self {
        self.callback = callback;

        self
    }
    


    pub fn instantiate(&'a self, shell: &'a Shell<S>, arg_list: Vec<ArgToken>) -> Result<CommandInstance<S>, CommandError>{
        if arg_list.len() > self.arg_list.len() { 
            return Err(CommandError::TooManyArguments(self.name.to_string(), self.arg_list.len(), arg_list.len())) 
        }

        else if arg_list.len() < self.arg_list.len() { 
            return Err(CommandError::TooFewArguments(self.name.to_string(), self.arg_list.len(), arg_list.len())) 
        }

        let mut inst_arg_list: Vec<EvaluatedArg> = vec![];
        let mut arg_list = arg_list.iter();

        for arg in &self.arg_list {
            match arg.evaluate(arg_list.next().unwrap()) {
                Ok(eval) => inst_arg_list.push(eval),
                Err(e) => return Err(e)
            }
        }


        Ok(CommandInstance::<S>::new(shell, inst_arg_list, self.callback))
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn arg_list(&self) -> &Vec<ArgType> {
        &self.arg_list
    }

    pub fn description(&self) -> &'static str {
        self.description
    }
}