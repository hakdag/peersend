use crate::command::Command;
use std::env;
use std::io::Error;

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self) -> Result<Command, Error> {
        let args: Vec<String> = env::args().collect();
        let mut count = args.len();
        if count < 2 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "not enough arguments"));
        }

        let mut current: usize = 1;
        while current < count {
            let arg = Self::getCurrentArg(&args, current);
            match arg {
                "help" => return Result::Ok(Command::new("Help".to_string())),
                other => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "unknown command"))
            };
            current += 1;
        }
        let cmd = Command::new("test".to_string());
        Result::Ok(cmd)
    }

    fn getCurrentArg(args: &Vec<String>, index: usize) -> &str {
        match args.get(index) {
            Some(arg) => arg,
            None => ""
        }
    }
}
