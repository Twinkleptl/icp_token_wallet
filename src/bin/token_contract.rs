use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct Token {
    owner: String,
    balance: u64,
}

#[derive(Default)]
struct TokenContract {
    tokens: HashMap<String, Token>,
}

impl TokenContract {
    fn new() -> Self {
        Self::default()
    }

    fn get_balance(&self, owner: &String) -> u64 {
        self.tokens.get(owner).map_or(0, |token| token.balance)
    }

    fn transfer(&mut self, from: String, to: String, amount: u64) -> Result<(), String> {
        let sender_balance = self.get_balance(&from);
        if sender_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        self.tokens.entry(from.clone()).and_modify(|e| e.balance -= amount);
        self.tokens.entry(to.clone()).and_modify(|e| e.balance += amount).or_insert(Token { owner: to.clone(), balance: amount });

        Ok(())
    }
}

static mut CONTRACT: Option<TokenContract> = None;

#[init]
fn init() {
    unsafe {
        CONTRACT = Some(TokenContract::new());
    }
}

#[query]
fn balance_of(owner: String) -> u64 {
    unsafe { CONTRACT.as_ref().unwrap().get_balance(&owner) }
}

#[update]
fn transfer(from: String, to: String, amount: u64) -> Result<(), String> {
    unsafe { CONTRACT.as_mut().unwrap().transfer(from, to, amount) }
}