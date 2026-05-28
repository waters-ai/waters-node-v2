// src/agent/agent.rs
// Agent module for node 2.0

use crate::bridge::Bridge;
use crate::wallet::Wallet;
use std::collections::HashMap;

pub struct Agent {
    pub id: String,
    pub wallet: Wallet,
    pub bridges: HashMap<String, Bridge>,
    pub groups: Vec<String>,
}

impl Agent {
    pub fn new(id: &str) -> Self {
        Agent {
            id: id.to_string(),
            wallet: Wallet::new(),
            bridges: HashMap::new(),
            groups: Vec::new(),
        }
    }

    pub fn add_bridge(&mut self, name: &str, bridge: Bridge) {
        self.bridges.insert(name.to_string(), bridge);
    }

    pub fn join_group(&mut self, group_id: &str) {
        self.groups.push(group_id.to_string());
    }
}
