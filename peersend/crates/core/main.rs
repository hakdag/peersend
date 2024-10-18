use std::{process::ExitCode};
mod parser;
mod command;
use crate::parser::Parser;
use crate::command::Command;
use std::io::Error;

fn main() -> ExitCode {
    /*
    Command structrure:
    peersend [main command] [command arguments] [optional arguments]

    Example:
    peersend test
               |
               -> command
    Runs test command. This commands prints out "Test" text.

    peersend exists test.txt
                |       |
                |       -> argument
                -> command
    Runs exists command. This command checks if the file exists in the folder.
    */
    let parser = Parser::new();
    match run(parser.parse()) {
        Ok(res) => {
            println!("{}", res.message);
            ExitCode::from(0)
        },
        Err(error) => {
            println!("{:#}", error);
            ExitCode::from(2)
        }
    }
}

fn run(result: Result<Command, Error>) -> Result<ExecutionResult, Error> {
    println!("Running command: {:?}", result);
    match result {
        Ok(cmd) => Ok(ExecutionResult::new(cmd)),
        Err(error) => Err(error)
    }
}

struct ExecutionResult {
    pub message: String,
    pub command: Command
}

impl ExecutionResult {
    pub fn new(cmd: Command) -> ExecutionResult {
        ExecutionResult {
            message: "ExecutionResult".to_string(),
            command: cmd
        }
    }
}