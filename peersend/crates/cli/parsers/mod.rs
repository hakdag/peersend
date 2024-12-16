pub mod help_parser;
pub mod version_parser;
pub mod create_user_parser;
pub mod login_parser;
pub mod register_device_parser;
pub mod send_file_parser;
pub mod listen_parser;

pub fn get_arg(args: &Vec<String>, index: usize) -> &str {
    match args.get(index) {
        Some(arg) => arg,
        None => ""
    }
}
