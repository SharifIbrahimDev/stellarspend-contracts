use soroban_sdk::{contractimpl, contracttype, Env, String, Symbol};
use crate::RagContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccessPolicy {
    OwnerOnly,
    MembersOnly,
    Public,
}

#[contractimpl]
impl RagContract {
    pub fn set_access_policy(env: Env, resource_id: String, policy: AccessPolicy) {
        let key = (Symbol::new(&env, "access_policy"), resource_id.clone());
        env.storage().instance().set(&key, &policy);
    }

    pub fn get_access_policy(env: Env, resource_id: String) -> AccessPolicy {
        let key = (Symbol::new(&env, "access_policy"), resource_id);
        env.storage().instance().get(&key).unwrap_or(AccessPolicy::OwnerOnly)
    }
}
