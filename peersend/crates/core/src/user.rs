use std::io::Error;

use redis::{from_redis_value, FromRedisValue, ToRedisArgs};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use validify::Validify;
use crate::{device::Device, requests::check_user::CheckUserRequest};

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct User {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,

    pub devices: Vec<Device>,
}

impl User {
    pub fn new(username: String, email: String, devices: Option<Vec<Device>>) -> Self {
        if devices.is_some() {
            Self { username, email, devices: devices.unwrap() }
        }
        else {
            Self { username, email, devices: Vec::new() }
        }
    }

    pub fn user_name(&self) -> &String {
        &self.username
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn has_device(&self, arg_source_device: &String) -> bool {
        let mut n = 0;
        while n < self.devices.len() {
            let device = self.devices.get(n).unwrap();
            if device.devicename == *arg_source_device {
                return true;
            }

            n += 1;
        }

        false
    }

    pub fn get_device_by_name(&self, device_name: &String) -> Option<&Device> {
        let mut n = 0;
        while n < self.devices.len() {
            let device = self.devices.get(n).unwrap();
            if device.devicename == *device_name {
                return Some(device);
            }

            n += 1;
        }

        None
    }
}

impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W) where W: ?Sized + redis::RedisWrite {
        let bytes = serialize(self).unwrap();
        out.write_arg(&bytes);
    }
}

impl FromRedisValue for User {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let r: Vec<u8> = from_redis_value(v)?;
        let obj: User = match deserialize(&r) {
            Ok(u) => u,
            Err(e) => {
                panic!("Error when deserializing user: {}", e.to_string());
            },
        };
        Ok(obj)
    }
}

pub trait UsersAccessable {
    fn get_token(&self) -> Result<String, Error>;
    fn get_user(&self) -> Result<User, Error>;
    fn check_user(&self, request: CheckUserRequest) -> Result<(), Error>;
}