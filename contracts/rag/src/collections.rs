use soroban_sdk::{contractimpl, Address, Env, String, Map};
use crate::RagContract;

#[contractimpl]
impl RagContract {
    pub fn add_member(env: Env, collection_id: String, member: Address) {
        member.require_auth();
        
        let mut members: Map<Address, bool> = env.storage().instance().get(&collection_id).unwrap_or(Map::new(&env));
        if !members.contains_key(member.clone()) {
            members.set(member, true);
            env.storage().instance().set(&collection_id, &members);
        } else {
            panic!("Member already exists");
        }
    }

    pub fn remove_member(env: Env, collection_id: String, member: Address) {
        member.require_auth();

        let mut members: Map<Address, bool> = env.storage().instance().get(&collection_id).unwrap_or(Map::new(&env));
        if members.contains_key(member.clone()) {
            members.remove(member);
            env.storage().instance().set(&collection_id, &members);
        }
    }

    pub fn check_membership(env: Env, collection_id: String, member: Address) -> bool {
        let members: Map<Address, bool> = env.storage().instance().get(&collection_id).unwrap_or(Map::new(&env));
        members.contains_key(member)
    }
}
