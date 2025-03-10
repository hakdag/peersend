use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerSession {
    pub id: String,
    pub email: String,
    pub device_name: String,
    pub ip_address: String,
    pub mac: String,
    pub expires: usize,
}

impl PeerSession {
    pub fn new(email: String, device_name: String, ip_address: String, mac: String) -> Self { 
        let id = Uuid::new_v4().to_string();
        let date_time = Utc::now();
        let later = date_time + chrono::Duration::hours(2);
        let expires: usize = usize::try_from(later.timestamp()).unwrap();
        Self { id, email, device_name, ip_address, mac, expires }
    }
}
