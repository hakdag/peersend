use redis::{from_redis_value, FromRedisValue, ToRedisArgs};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerSession {
    pub id: String,
    pub email: String,
    pub devices: Vec<PeerSessionDevice>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerSessionDevice {
    pub device_name: String,
    pub ip_address: String,
    pub mac: String,
    pub expires: usize,
}

impl PeerSession {
    pub fn new(email: &String, device_name: String, ip_address: String, mac: String) -> Self { 
        let id = Uuid::new_v4().to_string();
        let device = PeerSession::create_device(device_name, ip_address, mac);
        let devices = Vec::from([device]);
        Self { id, email: email.to_owned(), devices }
    }
    
    pub(crate) fn get_device(&self, device_name: String) -> Option<PeerSessionDevice> {
        match self.devices.iter().find(|d| d.device_name == device_name) {
            Some(d) => Some(d.to_owned()),
            None => None,
        }
    }

    fn create_device(device_name: String, ip_address: String, mac: String) -> PeerSessionDevice {
        let date_time = Utc::now();
        let later = date_time + chrono::Duration::hours(2);
        let expires: usize = usize::try_from(later.timestamp()).unwrap();
        PeerSessionDevice { device_name, ip_address, mac, expires }
    }

    pub fn add_device(&mut self, device_name: String, ip_address: String, mac: String) {
        // dont add same device twice
        let existing_device = self.devices.iter().find(|d| d.device_name == device_name);
        if existing_device.is_none() {
            let device = PeerSession::create_device(device_name, ip_address, mac);
            self.devices.push(device);
        }
    }
}

impl ToRedisArgs for PeerSession {
    fn write_redis_args<W>(&self, out: &mut W) where W: ?Sized + redis::RedisWrite {
        let bytes = serialize(self).unwrap();
        out.write_arg(&bytes);
    }
}

impl FromRedisValue for PeerSession {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let r: Vec<u8> = from_redis_value(v)?;
        let obj: PeerSession = deserialize(&r).unwrap();
        Ok(obj)
    }
}
