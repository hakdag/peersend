use core::device::Device;
use core::requests::create_user::CreateUserRequest;
use core::requests::login::LoginRequest;
use core::user::User;
use std::io::{self, BufRead, Write};
use std::fs::{self, File, OpenOptions};
use std::path::Path;

use crate::models::error::FileSystemError;

pub struct FileAccess {}

impl FileAccess {
    pub fn new() -> Self { Self {} }

    pub fn write_user(&self, request: CreateUserRequest) -> Result<(), FileSystemError> {
        // check if a file exists with same email
        let res = match fs::exists(&request.email) {
            Ok(r) => r,
            Err(e) => return Err(e.into()),
        };

        if res == true {
            return Err(FileSystemError::new(request.email, "User with same email already exists.".to_string()));
        }

        // create a folder with name email of the user
        let _ = match fs::create_dir(&request.email) {
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        };
        self.write_to_file(format!("{}/username", request.email), request.username.as_bytes())?;
        self.write_to_file(format!("{}/password", request.email), request.password.as_bytes())?;
        self.write_to_file(format!("{}/email", request.email), request.email.as_bytes())?;

        let mut devices_f = fs::File::create(format!("{}/devices", request.email))?;
        for device in &request.devices {
            devices_f.write_all(device.devicename.as_bytes())?;
            devices_f.write("\n".to_string().as_bytes())?;
        };

        // (optional) hash file contents and store in a separate file: <email>_hash

        Ok(())
    }

    fn write_to_file(&self, path: String, data: &[u8]) -> Result<(), FileSystemError> {
        let mut file = fs::File::create(path)?;
        Ok(file.write_all(data)?)
    }

    pub fn read_user(&self, email: String) -> Result<User, FileSystemError> {
        // check if a file exists with same email
        let res = fs::exists(&email)?;

        if res == false {
            return Err(FileSystemError::new(email, "User could not be found.".to_string()));
        }

        Ok(self.constract_user(email)?)
    }
    
    pub(crate) fn get_user_password(&self, request: &LoginRequest) -> Result<String, FileSystemError> {
        let res = fs::exists(&request.email)?;
        if res == false {
            return Err(FileSystemError::new(format!("{}", request.email), "User could not be found.".to_string()));
        }

        let password = fs::read_to_string(format!("{}/password", request.email))?;
        Ok(password)
    }

    fn constract_user(&self, id: String) -> Result<User, FileSystemError> {
        let username = fs::read_to_string(format!("{}/username", id))?;
        let email = fs::read_to_string(format!("{}/email", id))?;

        // read devices
        let mut devices: Vec<Device> = Vec::new();
        if let Ok(lines) = self.read_lines(format!("{}/devices", id)) {
            for devicename in lines.map_while(Result::ok) {
                let device = Device::new(devicename, None);
                devices.push(device);
            }
        }

        return Ok(User::new(username, email, Some(devices)))
    }

    fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    
    pub(crate) fn add_device_to_user(&self, user: &User, devicename: &str) -> Result<(), FileSystemError> {
        let mut devices_f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/devices", user.email))
            .unwrap();

        devices_f.write_all(devicename.as_bytes())?;
        devices_f.write("\n".to_string().as_bytes())?;
        Ok(())
    }
}
