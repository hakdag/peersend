use std::process::ExitCode;
mod parser;
mod command_executor;
mod parsers;
use crate::parser::Parser;
use crate::command_executor::CommandExecutor;
use core::command;

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
    match Parser::parse() {
        Ok(cmd) => {
            match CommandExecutor::execute(&cmd) {
                Ok(res) => {
                    println!("{}", res);
                    ExitCode::from(0)
                },
                Err(error) => {
                    println!("{:#}", error);
                    ExitCode::from(2)
                }
            }
        },
        Err(error) => {
            println!("{:#}", error);
            ExitCode::from(2)
        }
    }
}
