#[derive(Debug)]
pub struct Command {
    pub name: String
}

impl Command {
    pub fn new(name: String) -> Command {
        Command { name: name }
    }
}

#[derive(Debug)]
pub struct CommandArgument {}

