use redis::{from_redis_value, FromRedisValue, ToRedisArgs};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use validify::Validify;

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct Device {
    #[validate(length(min = 3))]
    pub devicename: String,
}

impl Device {
    pub fn new(devicename: String) -> Self {
        Self { devicename }
    }
}

impl ToRedisArgs for Device {
    fn write_redis_args<W>(&self, out: &mut W) where W: ?Sized + redis::RedisWrite {
        let bytes = serialize(self).unwrap();
        out.write_arg(&bytes);
    }
}

impl FromRedisValue for Device {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let r: Vec<u8> = from_redis_value(v)?;
        let obj: Device = deserialize(&r).unwrap();
        Ok(obj)
    }
}