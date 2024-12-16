use std::io::Error;

#[derive(Debug)]
pub struct HelpService {}

impl HelpService {
    pub fn run() -> Result<String, Error> {
        let mut parts = vec!["peersend [command] [arguments]"];
        parts.push("Commands and arguments:");
        parts.push("help");
        parts.push("version");
        parts.push("create-user [username] [password] [email]");
        parts.push("login [username] [password]");
        parts.push("register [devicename]");
        parts.push("send [filepath] [source device name] [target device name]");

        let output = parts.join("\n");
        Result::Ok(output)
    }
}