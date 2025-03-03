use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerSession {
    pub id: String,
    pub user_id: String,
    pub device_name: String,
    pub ip_address: String,
}

impl PeerSession {
    pub fn new(user_id: String, device_name: String, ip_address: String) -> Self { 
        let id = Uuid::new_v4().to_string();
        Self { id, user_id, device_name, ip_address }
    }
}
