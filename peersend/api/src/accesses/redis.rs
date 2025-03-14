use std::io::Error;
use redis::{Client, Commands, FromRedisValue, ToRedisArgs};

pub struct RedisAccess {
    client: Client,
}

impl RedisAccess {
    pub fn new() -> Result<RedisAccess, Error> {
        let client = match Client::open("redis://127.0.0.1:6379") {
            Ok(c) => c,
            Err(e) => return Result::Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, format!("Could not reach Redis. {}", e.to_string()))),
        };
        Result::Ok(RedisAccess { client: client })
    }
    pub fn set<TObj>(&self, key: &String, obj: TObj) -> Result<(), Error> where TObj: ToRedisArgs {
        match self.client.get_connection() {
            Ok(mut con) => {
                redis::cmd("SET").arg(key).arg(obj).exec(&mut con).unwrap();
                return Result::Ok(());
            },
            Err(_) => Result::Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection to redis failed.")),
        }
    }
    
    pub fn get<TObj>(&self, key: &String) -> Result<Option<TObj>, Error> where TObj: FromRedisValue {
        match self.client.get_connection() {
            Ok(mut con) => {
                match con.get(key) {
                    Ok(obj) => Ok(obj),
                    Err(_) => Ok(None),
                }
            },
            Err(_) => Result::Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection to redis failed.")),
        }
    }
}
