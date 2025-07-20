# Rust Banking System

This project is a simple banking system implemented in Rust. It demonstrates the use of **traits**, **structs**, and **method implementations** to simulate basic banking operations such as deposits, withdrawals, and balance checks.

---

## Features
- Create bank accounts with an account number, holder name, and initial balance.
- Deposit and withdraw money with simple validation.
- Display the current balance of an account.
- Implements **traits with associated types** to define a generic `Account` behavior.

---

## Code Overview
The project defines:
- **`BankAccount` struct:** Represents an account with fields for `account_number`, `holder_name`, and `balance`.
- **`Account` trait:** Provides abstract methods for deposit, withdrawal, and balance operations.
- **Implementation of `Account` for `BankAccount`:** Implements the required methods to manage account balances.
- **Main function:** Demonstrates the functionality by creating two accounts (`Alice` and `Bob`) and performing a few transactions.



