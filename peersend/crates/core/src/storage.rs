use std::io::Error;

use redis::{FromRedisValue, ToRedisArgs};

pub trait StorageAccess {
    fn set<TObj>(&self, key: String, obj: TObj) -> Result<(), Error> where TObj: ToRedisArgs;
    fn get<TObj>(&self, key: String) -> Result<TObj, Error> where TObj: FromRedisValue;
}