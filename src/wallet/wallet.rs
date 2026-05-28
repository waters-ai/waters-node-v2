// src/wallet/wallet.rs
// Wallet management module for node 2.0

use ed25519_dalek::Signer;
use slh_dsa::sign::Signer;
use std::collections::HashMap;

pub struct Wallet {
    pub address: String,
    pub ed25519_sk: ed25519_dalek::SigningKey,
    pub slh_dsa_sk: slh_dsa::sign::SigningKey,
    pub balance: u64,
}

impl Wallet {
    pub fn new() -> Self {
        let (ed_sk, _) = ed25519_dalek::SigningKey::generate(&mut rand::thread_rng());
        let (pq_sk, _) = slh_dsa::sign::SigningKey::generate(&mut rand::thread_rng());
        Wallet {
            address: format!("{}", ed_sk.verifying_key()),
            ed25519_sk: ed_sk,
            slh_dsa_sk: pq_sk,
            balance: 0,
        }
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let ed_sig = self.ed25519_sk.sign(data);
        let pq_sig = self.slh_dsa_sk.sign(data);
        [ed_sig.as_ref(), pq_sig.as_ref()].concat()
    }
}
