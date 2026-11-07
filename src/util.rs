//! teamviewer model — remote desktop software.

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub partner_id: String,
    pub status: String,
    pub direction: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub sessions: Vec<Session>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { sessions: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn connect(&mut self, name: &str, partner_id: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.sessions.push(Session { id: id.clone(), name: name.into(), partner_id: partner_id.into(), status: "connecting".into(), direction: "outgoing".into() });
        id
    }

    pub fn disconnect(&mut self, id: &str) -> bool {
        if let Some(s) = self.sessions.iter_mut().find(|s| s.id == id) { s.status = "disconnected".into(); true } else { false }
    }

    pub fn list(&self) -> &[Session] { &self.sessions }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_connect_disconnect() {
        let mut t = Model::new();
        let id = t.connect("partner", "123456789");
        assert!(t.disconnect(&id));
    }
}
