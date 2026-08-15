mod bank_account;
use bank_account::{BankAccount, Date};

fn main() {
    let mut account= BankAccount::new(
        String::from("John Doe"),
        Date { day: 15, month: 6, year: 1990 },
        123456789,
        1000.0,
        String::from("john.doe@example.com"),
        String::from("123-456-7890"),
        String::from("password123")
    );
    menu(&mut account);
}
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn confirm_balance(balance: f64) {
    if balance < 0.0 {
        println!("Your balance is negative: ${}", balance);
    } else {
        println!("Your balance is positive: ${}", balance);
    }
}

fn menu(account: &mut BankAccount) {
    loop {
        println!("Welcome to the Bank Account Management System");
        println!("Please choose an option:");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Check Balance");
        println!("4. Check Account Details");
        println!("5. Set Password");
        println!("6. Change Account Details");
        println!("7. Get Account Details");
        println!("8. Get Transaction History");
        println!("9. Send Money");
        println!("10. Exit");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 10.");
                continue;
            }
        };

        match choice {
            1 => handle_deposit(account),
            2 => handle_withdraw(account),
            3 => {
                let balance = account.get_balance();
                println!("Current balance is: ${}", balance);
                confirm_balance(balance);
            }
            4 => account.get_account_details(),
            5 => handle_password_change(account),
            6 => change_account_details(account),
            7 => account.get_account_details(),
            8 => {
                let transaction_history = account.get_transaction_history();
                println!("Transaction History:");
                if transaction_history.is_empty() {
                    println!("No transactions found.");
                } else {
                    for transaction in transaction_history {
                        println!(
                            "From: {}, To: {}, Amount: ${}, Fee: ${}, Timestamp: {:?}",
                            transaction.from_account,
                            transaction.to_account,
                            transaction.amount,
                            transaction.fee,
                            transaction.timestamp
                        );
                    }
                }
            }
            9 => handle_send_money(account),
            10 => break,
            _ => {
                println!("Invalid input. Please enter a number between 1 and 10.");
            }
        }
    }
}

fn handle_deposit(account: &mut BankAccount) {
    let is_pin_correct = check_pin(account);
    if(!is_pin_correct){
        println!("Incorrect PIN. Withdrawal failed.");
        return;
    }
    println!("Enter amount to deposit:");
    let mut amt_str = String::new();    
    std::io::stdin().read_line(&mut amt_str).expect("Failed to read line");  
    
    match amt_str.trim().parse::<f64>() {
        Ok(amount) => {
            account.deposit(amount);
        },
        Err(_) => {
            println!("Invalid amount. Please enter a valid number.");
        }
    }
} // Only one closing brace needed here

fn handle_withdraw(account:&mut BankAccount){
    let is_pin_correct = check_pin(account);
    if(!is_pin_correct){
        println!("Incorrect PIN. Withdrawal failed.");
        return;
    }
    println!("Enter amount you wish to withdraw");
    let mut withdraw_amt = String::new();
    std::io::stdin()
        .read_line(&mut withdraw_amt)
        .expect("Failed to read input from user");
    match withdraw_amt.trim().parse::<f64>() {
        Ok(amount) => {
            account.withdraw(amount);
        },
        Err(_) => {
            println!("Invalid amount. Please enter a valid number.");
        }
    }
}

fn handle_password_change(account: &mut BankAccount) {
    println!("Enter your current password:");
    let mut current_password = String::new();
    std::io::stdin().read_line(&mut current_password).expect("Failed to read line");
    let current_password = current_password.trim();

    if current_password == account.bank_password {
        println!("Enter your new password:");
        let mut new_password = String::new();
        std::io::stdin().read_line(&mut new_password).expect("Failed to read line");
        let new_password = new_password.trim();

        account.bank_password = new_password.to_string();
        println!("Password changed successfully.");
    } else {
        println!("Incorrect current password. Password change failed.");
    }
}

fn check_pin(account: &BankAccount) -> bool {
    let mut pin = String::new();
    println!("Enter your PIN:");
    std::io::stdin().read_line(&mut pin).expect("Failed to read line");
    let pin = pin.trim();
    pin == account.bank_password
}

fn change_account_details(account: &mut BankAccount) {
    println!("Enter new name (leave blank to keep current):");
    let mut new_name = String::new();
    std::io::stdin().read_line(&mut new_name).expect("Failed to read line");
    let new_name = new_name.trim();
    if !new_name.is_empty() {
        account.name = new_name.to_string();
    }

    println!("Enter new email (leave blank to keep current):");
    let mut new_email = String::new();
    std::io::stdin().read_line(&mut new_email).expect("Failed to read line");
    let new_email = new_email.trim();
    if !new_email.is_empty() {
        account.email = new_email.to_string();
    }

    println!("Enter new mobile number (leave blank to keep current):");
    let mut new_mobile_number = String::new();
    std::io::stdin().read_line(&mut new_mobile_number).expect("Failed to read line");
    let new_mobile_number = new_mobile_number.trim();
    if !new_mobile_number.is_empty() {
        account.mobile_number = new_mobile_number.to_string();
    }

    println!("Account details updated successfully.");
}

fn handle_send_money(account: &mut BankAccount) {
    let is_pin_correct = check_pin(account);
    if(!is_pin_correct){
        println!("Incorrect PIN. Transfer failed.");
        return;
    }
    println!("Enter recipient's account number:");
    let mut recipient_account_number_str = String::new();
    std::io::stdin().read_line(&mut recipient_account_number_str).expect("Failed to read line");
    let recipient_account_number: u32 = match recipient_account_number_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid account number. Please enter a valid number.");
            return;
        }
    };

    // For simplicity, we will create a new BankAccount for the recipient.
    // In a real application, you would look up the recipient's account in a database.
    let mut recipient_account = BankAccount::new(
        String::from("Recipient Name"),
        Date { day: 1, month: 1, year: 2000 },
        recipient_account_number,
        0.0,
        String::from("recipient@example.com"),
        String::from("9876543210"),
        String::from("recipient_password")
    );
    account.send_money(&mut recipient_account, 100.0); // Example transfer amount   

}