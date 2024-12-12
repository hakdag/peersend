use core::command::CommandArgument;

pub mod help;
pub mod version;
pub mod create_user;
pub mod login;
pub mod register_device;
pub mod send_file;
pub mod jwt;
pub mod file;

pub fn get_arg(arguments: &Vec<CommandArgument>, index: usize) -> String {
    match arguments.get(index) {
        Some(arg) => arg.name.clone(),
        None => String::new(),
    }
}
