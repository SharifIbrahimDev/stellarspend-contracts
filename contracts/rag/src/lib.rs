#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

mod collections;
mod access;
mod documents;

pub use collections::*;
pub use access::*;
pub use documents::*;

#[contract]
pub struct RagContract;

#[contractimpl]
impl RagContract {
    pub fn init(env: Env) {
        // Initialize the contract
    }
}
