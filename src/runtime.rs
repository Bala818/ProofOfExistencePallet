mod proof;

use proof::{Call, ProofOfExistence};

#[derive(Debug)]
pub struct Runtime<AccountId, Content> {
    pub proof: ProofOfExistence<AccountId, Content>,
}

impl<AccountId, Content> Runtime<AccountId, Content>
where
    AccountId: Clone + Ord,
    Content: Clone + Ord,
{
    pub fn new() -> Self {
        Self {
            proof: ProofOfExistence::new(),
        }
    }

    pub fn execute(&mut self, call: Call<AccountId, Content>) -> Result<(), String> {
        self.proof.dispatch(call)
    }
}
