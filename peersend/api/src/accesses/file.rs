use core::device::Device;
use core::requests::create_user::CreateUserRequest;
use core::requests::login::LoginRequest;
use core::user::User;
use std::io::{self, BufRead, Read, Write};
use std::fs::{self, File};
use std::path::Path;

use crate::errors::file_system::FileSystemError;

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
        for entry in fs::read_dir(format!("{}/devices", id))? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let mut file = File::open(&path)?;
                let device_name = &path.file_name().unwrap().to_str().unwrap();
                let mut mac = String::new();
                file.read_to_string(&mut mac);
                let device = Device::new(device_name.to_string(), Some(mac));
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
    
    pub(crate) fn add_device_to_user(&self, user: &User, devicename: &str, mac: &str) -> Result<(), FileSystemError> {
        let devices_path = format!("{}/devices", user.email);
        let res = match fs::exists(&devices_path) {
            Ok(r) => r,
            Err(e) => return Err(e.into()),
        };
        if res == false {
            let _ = match fs::create_dir(&devices_path) {
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            };
        }

        let mut devices_f = fs::File::create(format!("{}/{}", devices_path, devicename))?;
        devices_f.write_all(mac.as_bytes())?;

        Ok(())
    }
}
