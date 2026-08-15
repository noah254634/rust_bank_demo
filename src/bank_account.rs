use std::time::SystemTime;

pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

pub struct Transaction {
    pub from_account: u32,
    pub to_account: u32,
    pub amount: f64,
    pub fee: f64,
    pub timestamp: SystemTime,
}
pub struct BankAccount {
    pub name: String,
    pub dob: Date,
    pub account_number: u32,
    pub balance: f64,
    pub email: String,
    pub mobile_number: String,
    pub bank_password: String,
    pub transaction_history: Vec<Transaction>,
}

impl BankAccount {
    pub fn new(name: String, dob: Date, account_number: u32, balance: f64, email: String, mobile_number: String, bank_password: String) -> Self {
        BankAccount {
            name,
            dob,
            account_number,
            balance,
            email,
            mobile_number,
            bank_password,
            transaction_history: Vec::new(),
        }
    }
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited ${}. New balance is ${}.", amount, self.balance);
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!("Withdrew ${}. New balance is ${}.", amount, self.balance);
        } else if amount > self.balance {
            println!("Insufficient funds. Current balance is ${}.", self.balance);
        } else {
            println!("Withdrawal amount must be positive.");
        }
    }
    pub fn send_money(&mut self, recipient: &mut BankAccount, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            recipient.balance += amount;
            println!("Sent ${} to {}. New balance is ${}.", amount, recipient.name, self.balance);
        } else if amount > self.balance {
            println!("Insufficient funds. Current balance is ${}.", self.balance);
        } else {
            println!("Transfer amount must be positive.");
        }
        let transaction = Transaction {
            from_account: self.account_number,
            to_account: recipient.account_number,
            amount,
            fee: 0.0, // Assuming no fee for simplicity
            timestamp: SystemTime::now(),
        };
        self.transaction_history.push(transaction);
    }
    pub fn get_balance(&self) -> f64 {
        self.balance
    }
    pub fn get_transaction_history(&self) -> &Vec<Transaction> {
        &self.transaction_history
    }
    pub fn get_account_number(&self) -> u32 {
        self.account_number
    }
    pub fn get_account_details(&self) {
        println!("Account Holder: {}", self.name);
        println!("Date of Birth: {:02}/{:02}/{}", self.dob.day, self.dob.month, self.dob.year);
        println!("Account Number: {}", self.account_number);
        println!("Balance: ${}", self.balance);
        println!("Email: {}", self.email);
        println!("Mobile Number: {}", self.mobile_number);
    }

   
}
