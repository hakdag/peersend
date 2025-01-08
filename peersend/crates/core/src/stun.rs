use std::net::SocketAddr;

pub trait STUNAccessible {
    fn discover_public_address(&self) -> Result<SocketAddr, Box<dyn std::error::Error>>;
}