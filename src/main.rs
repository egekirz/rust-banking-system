fn main() {
   let mut  alice = BankAccount {
        account_number: 2093453,
        holder_name: String::from("Alice"),
        balance: 260.00,
   };

   let mut bob= BankAccount {
        account_number: 2092456,
        holder_name: String::from("Bob"),
        balance: 140.00,
   };

    alice.deposit(42.02);
    bob.withdraw(42.02);
    alice.balance();
    bob.balance();
}

trait Account {
    type Balance;

    fn deposit(&mut self, deposit: Self:: Balance);
    fn withdraw(&mut self, withdraw: Self:: Balance);
    fn balance(&mut self);
}

struct BankAccount{
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    
    type Balance = f64;

    fn deposit(&mut self, deposit: Self::Balance) {
        self.balance += deposit;
        println!(
            "{} deposited {:.2}$.",
            self.holder_name, deposit
        )
    }

    fn withdraw(&mut self, withdraw: Self:: Balance) {
        if self.balance >= withdraw {
            self.balance -= withdraw;
            println!(
                "{} withdrew {:.2}$.",
                self.holder_name, withdraw
            );

        }else{
            println!(
                "{} tried to withdraw {:.2}$ but has insufficien funds.",
                self.holder_name, withdraw
            )
        }
    }

    fn balance(&mut self ) {
        println!(
            "Dear,{}; the balance of your account whose account number is {}, {:.2}$.  ",
            self.holder_name, self.account_number, self.balance
        ) 
    }
   
}