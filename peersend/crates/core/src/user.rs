use redis::{from_redis_value, FromRedisValue, ToRedisArgs};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use validify::Validify;

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct User {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String, email: String) -> User {
        User { username, password, email }
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