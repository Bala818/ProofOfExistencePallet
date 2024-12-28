use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ProofOfExistence<AccountId, Content> {
    claims: BTreeMap<Content, AccountId>,
}

impl<AccountId, Content> ProofOfExistence<AccountId, Content>
where
    AccountId: Clone + Ord,
    Content: Clone + Ord,
{
    /// Create a new ProofOfExistence instance
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    /// Get the owner of a claim
    pub fn get_claim(&self, content: &Content) -> Option<&AccountId> {
        self.claims.get(content)
    }

    /// Create a claim for the provided content
    pub fn create_claim(&mut self, caller: AccountId, content: Content) -> Result<(), String> {
        if self.claims.contains_key(&content) {
            return Err("Claim already exists.".to_string());
        }
        self.claims.insert(content, caller);
        Ok(())
    }

    /// Revoke a claim for the provided content
    pub fn revoke_claim(&mut self, caller: AccountId, content: &Content) -> Result<(), String> {
        match self.claims.get(content) {
            Some(owner) if *owner == caller => {
                self.claims.remove(content);
                Ok(())
            }
            Some(_) => Err("Caller is not the owner of the claim.".to_string()),
            None => Err("Claim does not exist.".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Call<AccountId, Content> {
    CreateClaim { caller: AccountId, content: Content },
    RevokeClaim { caller: AccountId, content: Content },
}

impl<AccountId, Content> ProofOfExistence<AccountId, Content>
where
    AccountId: Clone + Ord,
    Content: Clone + Ord,
{
    pub fn dispatch(&mut self, call: Call<AccountId, Content>) -> Result<(), String> {
        match call {
            Call::CreateClaim { caller, content } => self.create_claim(caller, content),
            Call::RevokeClaim { caller, content } => self.revoke_claim(caller, &content),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_get_claim() {
        let mut proof = ProofOfExistence::<String, String>::new();
        assert!(proof
            .create_claim("Alice".to_string(), "Document1".to_string())
            .is_ok());
        assert_eq!(
            proof.get_claim(&"Document1".to_string()),
            Some(&"Alice".to_string())
        );
    }

    #[test]
    fn test_revoke_claim() {
        let mut proof = ProofOfExistence::<String, String>::new();
        proof
            .create_claim("Alice".to_string(), "Document1".to_string())
            .unwrap();
        assert!(proof
            .revoke_claim("Alice".to_string(), &"Document1".to_string())
            .is_ok());
        assert_eq!(proof.get_claim(&"Document1".to_string()), None);
    }

    #[test]
    fn test_revoke_claim_not_owner() {
        let mut proof = ProofOfExistence::<String, String>::new();
        proof
            .create_claim("Alice".to_string(), "Document1".to_string())
            .unwrap();
        assert!(proof
            .revoke_claim("Bob".to_string(), &"Document1".to_string())
            .is_err());
    }
}
