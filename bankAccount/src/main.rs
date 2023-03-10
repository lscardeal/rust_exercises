#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn withdraw(self, amount: f64) -> Self {
        Self { owner: self.owner, balance: self.balance - amount }
    }

    fn deposit(self, amount: f64) -> Self {
        Self { owner: self.owner, balance: self.balance + amount }
    }
}

fn main() {
    let acc: BankAccount = BankAccount { owner: String::from("Joel"), balance: 1000.00 };
    let acc = acc.withdraw(500.00);
    let acc = acc.deposit(200.00);

    println!("{:#?}", acc);
}