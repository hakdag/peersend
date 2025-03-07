use crate::models::peer_session::PeerSession;

pub struct SessionDB {
    sessions: Vec<PeerSession>
}

impl SessionDB {
    pub fn new() -> Self { Self { sessions: Vec::new() }}

    pub fn add(&mut self, session: PeerSession) {
        self.sessions.push(session)
    }

    pub fn get(&self, mac: String) -> Option<PeerSession> {
        let index = match self.sessions.iter().position(|session| session.mac == mac) {
            Some(i) => i,
            None => return None,
        };

        let session = &self.sessions[index];
        Some(session.clone())
    }

    pub fn count(&self) -> usize {
        self.sessions.len()
    }
}
