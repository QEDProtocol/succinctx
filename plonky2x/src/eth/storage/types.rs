use crate::vars::{BoolVariable, Bytes32Variable, U256Variable};

#[derive(Debug, Clone, Copy)]
pub struct ProofVariable {
    pub proof: Bytes32Variable,
}

#[derive(Debug, Clone, Copy)]
pub struct AccountVariable {
    pub balance: U256Variable,
    pub code_hash: Bytes32Variable,
    pub nonce: U256Variable,
    pub storage_hash: Bytes32Variable,
}

impl AccountVariable {
    pub fn serialize(&self) -> Vec<BoolVariable> {
        return vec![];
        // return self.code_hash.0[..].to_vec();
    }
}