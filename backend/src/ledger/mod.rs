// Ledger module for Double Entry Bookkeeping
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledger {
    transactions: Vec<Transaction>,
    balances: HashMap<String, i64>, // AccountID -> Balance
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            balances: HashMap::new(),
        }
    }

    pub fn post_transaction(
        &mut self,
        description: String,
        postings: Vec<Posting>,
    ) -> Result<(), String> {
        // 1. Verify Zero Sum
        let sum: i64 = postings.iter().map(|p| p.amount).sum();
        if sum != 0 {
            return Err(format!("Transaction is not zero-sum. Sum: {}", sum));
        }

        // 2. Create Transaction
        let tx = Transaction {
            id: Uuid::new_v4().to_string(),
            description,
            timestamp: chrono::Utc::now(),
            postings: postings.clone(),
        };

        // 3. Update Balances (Append-only conceptually, but we cache balances)
        for p in &postings {
            let entry = self.balances.entry(p.account.clone()).or_insert(0);
            *entry += p.amount;
        }

        // 4. Record
        self.transactions.push(tx);
        Ok(())
    }

    pub fn get_balance(&self, account: &str) -> i64 {
        *self.balances.get(account).unwrap_or(&0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub postings: Vec<Posting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Posting {
    pub account: String,
    pub amount: i64,
}
