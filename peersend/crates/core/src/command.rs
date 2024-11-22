#[derive(Debug)]
pub enum CommandType {
    Help,
    Version,
    CreateUser,
    Login,
    RegisterDevice,
    Send
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub command_type: CommandType,
}

impl Command {
    pub fn new(name: String, ct: CommandType) -> Command {
        Command { name: name, command_type: ct }
    }
}

#[derive(Debug)]
pub struct CommandArgument {}

