trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    #[allow(dead_code)]
    account_number: String,
    #[allow(dead_code)]
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account_1 = BankAccount {
        account_number: String::from("123"),
        holder_name: String::from("John Doe"),
        balance: 0.0,
    };

    let mut account_2 = BankAccount {
        account_number: String::from("456"),
        holder_name: String::from("Jane Doe"),
        balance: 0.0,
    };

    account_1.deposit(100.0);
    account_2.withdraw(50.0);

    println!("Account 1 balance: {}", account_1.balance());
    println!("Account 2 balance: {}", account_2.balance());
}
