# toy-blockchain-rs

**toy-blockchain-rs** is a lightweight blockchain simulator written in Rust, inspired by the modular architecture of [Substrate](https://substrate.dev/). This project provides a simplified environment for learning, experimentation, and prototyping blockchain logic, pallets (modules), and transaction handling, all without the overhead of a full production chain.

## Project Goals
- **Educational:** Make blockchain internals accessible and hackable for students, hobbyists, and researchers.
- **Modular:** Enable users to create, register, and experiment with custom pallets and transactions—similar to how Substrate's runtime modules work.
- **Lightweight:** Minimal dependencies, clear structure, and easy to understand core concepts.

## How It Works
- **Multi-Pallet System:**  
  Much like Substrate, core logic is organized into "pallets" (modules). Each pallet can define storage, functions, and hooks for handling transactions or custom logic. Pallets are registered at startup, and the system routes relevant calls to them.

- **Transaction Handling:**  
  Transactions are basic Rust structs or enums, validated and processed according to pallet logic. The system processes transactions in a loop, updating state as necessary.

- **Account Balances:**  
  The simulator tracks balances and ownership using simple maps, updating them as transactions are executed.

- **Macros for Boilerplate:**  
  Custom macros are included to make it easier to define and register new pallets or transaction handlers, reducing repetitive code.
  
## Directory Structure
toy-blockchain-rs/
├── macros/ # Rust macros for streamlining pallet/transaction definitions
├── src/ # Main logic, pallets, and blockchain state
├── Cargo.toml # Rust project configuration
└── README.md # Project information
