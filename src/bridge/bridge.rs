// src/bridge/bridge.rs
// Multi-Protocol Bridge module for node 2.0

use crate::agent::Agent;
use crate::wallet::Wallet;
use std::collections::HashMap;

pub struct Bridge {
    pub id: String,
    pub protocols: HashMap<String, String>,
    pub agent: Agent,
}

impl Bridge {
    pub fn new(id: &str, agent: Agent) -> Self {
        Bridge {
            id: id.to_string(),
            protocols: HashMap::new(),
            agent,
        }
    }

    pub fn add_protocol(&mut self, name: &str, config: &str) {
        self.protocols.insert(name.to_string(), config.to_string());
    }
}
