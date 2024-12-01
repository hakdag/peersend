use core::storage::StorageAccess;
use std::io::Error;
use redis::{Client, Commands, FromRedisValue, ToRedisArgs};

pub struct RedisCommunication {
    client: Client,
}

impl RedisCommunication {
    pub fn new() -> Result<RedisCommunication, Error> {
        let client = Client::open("redis://localhost:6379").unwrap();
        Result::Ok(RedisCommunication { client: client })
    }
}

impl StorageAccess for RedisCommunication {
    fn set<TObj>(&self, key: String, obj: TObj) -> Result<(), Error> where TObj: ToRedisArgs {
        match self.client.get_connection() {
            Ok(mut con) => {
                redis::cmd("SET").arg(key).arg(obj).exec(&mut con).unwrap();
                return Result::Ok(());
            },
            Err(_) => Result::Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection to redis failed.")),
        }
    }
    
    fn get<TObj>(&self, key: String) -> Result<TObj, Error> where TObj: FromRedisValue {
        match self.client.get_connection() {
            Ok(mut con) => {
                let obj: TObj = con.get(key).unwrap();
                return Result::Ok(obj);
            },
            Err(_) => Result::Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection to redis failed.")),
        }
    }
}