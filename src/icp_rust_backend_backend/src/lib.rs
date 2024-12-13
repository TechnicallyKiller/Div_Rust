// Main.rs

use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use ic_cdk::query;
use ic_cdk::update;
use std::cell::RefCell;
use std::collections::HashMap;

type Address = String;

#[derive(CandidType, Deserialize, Clone, Debug)]
struct TokenBalance {
    balance: u64,
}

thread_local! {
    static BALANCES: RefCell<HashMap<Address, TokenBalance>> = RefCell::new(HashMap::new());
}

#[update]
fn send_tokens(sender: Address, recipient: Address, amount: u64) -> String {
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        
        let sender_balance = balances.entry(sender.clone()).or_insert(TokenBalance { balance: 0 });
        if sender_balance.balance < amount {
            return "Insufficient balance".to_string();
        }

        sender_balance.balance -= amount;
        let recipient_balance = balances.entry(recipient.clone()).or_insert(TokenBalance { balance: 0 });
        recipient_balance.balance += amount;

        format!("Successfully transferred {} tokens from {} to {}.", amount, sender, recipient)
    })
}

#[update]
fn receive_tokens(recipient: Address, amount: u64) -> String {
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let recipient_balance = balances.entry(recipient.clone()).or_insert(TokenBalance { balance: 0 });
        recipient_balance.balance += amount;
        format!("Successfully received {} tokens for {}.", amount, recipient)
    })
}

#[query]
fn get_balance(address: Address) -> String {
    BALANCES.with(|balances| {
        let balances = balances.borrow();
        if let Some(balance) = balances.get(&address) {
            format!("The balance of {} is {} tokens.", address, balance.balance)
        } else {
            format!("No balance found for address: {}.", address)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_tokens() {
        let sender = "sender_address".to_string();
        let recipient = "recipient_address".to_string();

        receive_tokens(sender.clone(), 100);
        let result = send_tokens(sender.clone(), recipient.clone(), 50);

        assert_eq!(result, "Successfully transferred 50 tokens from sender_address to recipient_address.");
        assert_eq!(get_balance(sender), "The balance of sender_address is 50 tokens.");
        assert_eq!(get_balance(recipient), "The balance of recipient_address is 50 tokens.");
    }

    #[test]
    fn test_insufficient_balance() {
        let sender = "low_balance_address".to_string();
        let recipient = "recipient_address".to_string();

        receive_tokens(sender.clone(), 10);
        let result = send_tokens(sender.clone(), recipient.clone(), 50);

        assert_eq!(result, "Insufficient balance");
    }
}

fn main() {
    println!("This binary is a placeholder for the smart contract library.");
}

