use crate::token_contract::{balance_of, transfer};
use ic_cdk::export::candid::CandidType;
use ic_cdk_macros::update;

#[derive(CandidType)]
struct Wallet {
    owner: String,
    balance: u64,
}

impl Wallet {
    fn new(owner: String) -> Self {
        let balance = balance_of(owner.clone());
        Self { owner, balance }
    }

    fn send_tokens(&mut self, to: String, amount: u64) -> Result<(), String> {
        transfer(self.owner.clone(), to.clone(), amount)?;
        self.balance -= amount;
        Ok(())
    }

    fn receive_tokens(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn get_balance(&self) -> u64 {
        self.balance
    }
}

#[update]
fn create_wallet(owner: String) -> Wallet {
    Wallet::new(owner)
}