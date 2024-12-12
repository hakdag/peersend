use redis::{from_redis_value, FromRedisValue, ToRedisArgs};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use validify::Validify;
use crate::device::Device;

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct User {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String,

    pub devices: Vec<Device>,
}

impl User {
    pub fn new(username: String, password: String, email: String) -> User {
        User { username, password, email, devices: Vec::new() }
    }

    pub fn user_name(&self) -> &String {
        &self.username
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn has_device(&self, arg_source_device: String) -> bool {
        let mut n = 0;
        while n < self.devices.len() {
            let device = self.devices.get(n).unwrap();
            if device.devicename == arg_source_device {
                return true;
            }

            n += 1;
        }

        false
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
        let obj: User = deserialize(&r).unwrap();
        Ok(obj)
    }
}