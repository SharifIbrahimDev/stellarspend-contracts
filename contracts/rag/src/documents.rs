use soroban_sdk::{contractimpl, contracttype, Address, BytesN, Env, String, Symbol};
use crate::RagContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentMetadata {
    pub owner: Address,
    pub collection_id: String,
    pub content_hash: BytesN<32>,
    pub metadata_uri: String,
}

#[contractimpl]
impl RagContract {
    pub fn register_document(env: Env, doc_id: String, owner: Address, collection_id: String, content_hash: BytesN<32>, metadata_uri: String) {
        owner.require_auth();
        let key = (Symbol::new(&env, "document"), doc_id.clone());
        let metadata = DocumentMetadata { owner, collection_id, content_hash, metadata_uri };
        env.storage().instance().set(&key, &metadata);
    }

    pub fn get_document_owner(env: Env, doc_id: String) -> Address {
        let key = (Symbol::new(&env, "document"), doc_id);
        let metadata: DocumentMetadata = env.storage().instance().get(&key).unwrap();
        metadata.owner
    }

    pub fn transfer_document_ownership(env: Env, doc_id: String, new_owner: Address) {
        let key = (Symbol::new(&env, "document"), doc_id.clone());
        let mut metadata: DocumentMetadata = env.storage().instance().get(&key).unwrap();
        
        metadata.owner.require_auth();
        metadata.owner = new_owner.clone();
        
        env.storage().instance().set(&key, &metadata);
        
        env.events().publish(
            (Symbol::new(&env, "DocumentOwnershipTransferredEvent"), doc_id),
            new_owner
        );
    }
}
