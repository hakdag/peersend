#[derive(Debug)]
pub enum CommandType {
    Help,
    Version,
    CreateUser,
    Login,
    RegisterDevice,
    Listen,
    Send
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub command_type: CommandType,
    pub arguments: Option<Vec<CommandArgument>>
}

impl Command {
    pub fn new(name: String, ct: CommandType, args: Option<Vec<CommandArgument>>) -> Command {
        Command { name: name, command_type: ct, arguments: args }
    }
}

#[derive(Debug)]
pub struct CommandArgument {
    pub name: String
}

impl CommandArgument {
    pub fn new(name: String) -> CommandArgument {
        CommandArgument { name: name }
    }
}
