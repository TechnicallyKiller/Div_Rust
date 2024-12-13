# Rust Wallet for ICP Blockchain

## Overview
This project implements a wallet for the Internet Computer Protocol (ICP) blockchain. The wallet is developed using Rust and adheres to the IRCRC2 token standard, incorporating best practices in blockchain security.

## Features
- **Account Management**: Create and manage accounts.
- **Token Transactions**: Send, receive, and query token balances.
- **Security**: Implements cryptographic measures to secure transactions.

---

## Prerequisites

Ensure you have the following tools installed:
- Rust (version 1.71.0 or higher)
- Cargo
- Clang/GCC for building Rust projects
- ICP SDK (Internet Computer development environment)

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify Rust installation:
```bash
rustc --version
cargo --version
```

---

## Setup

1. **Clone the Repository**
   ```bash
   git clone https://github.com/your-username/icp-wallet.git
   cd icp-wallet
   ```

2. **Install Dependencies**
   Ensure all required dependencies are specified in `Cargo.toml`:
   ```toml
   [dependencies]
   ic-cdk = "0.15"
   candid = "0.10"
   serde = { version = "1.0", features = ["derive"] }
   serde_derive = "1.0"
   ```

   Run:
   ```bash
   cargo build
   ```

3. **Set Up the Linker**
   If you encounter linker errors, configure the linker in `~/.cargo/config.toml`:
   ```toml
   [target.x86_64-unknown-linux-gnu]
   linker = "/usr/bin/clang"
   ```

4. **Clean and Build**
   ```bash
   cargo clean
   cargo build
   ```

---

## Errors Faced and Resolutions

### 1. **Linking Error with `cc`**
   **Error:** Linking with `cc` failed due to a missing linker.
   **Solution:** Installed `clang` and updated the linker configuration.
   ```bash
   sudo apt-get install clang
   ```

### 2. **Manifest Key Warning**
   **Error:** `unused manifest key: build`
   **Solution:** Corrected the `Cargo.toml` configuration by removing unnecessary keys.

### 3. **Syntax Error in VERSION Script**
   **Error:** `/usr/bin/ld:/tmp/...: syntax error in VERSION script`
   **Solution:** Ensured that the linker was correctly set to `clang` in the Cargo configuration.

### 4. **Dependency Issues**
   **Error:** Missing dependency versions.
   **Solution:** Updated the `Cargo.toml` file with the correct versions of dependencies.

---

## Testing

1. **Run Unit Tests**
   ```bash
   cargo test
   ```

2. **Deploy Smart Contract**
   Use the ICP SDK to deploy and test the wallet on the blockchain.
   ```bash
   dfx deploy
   ```

3. **Execute Wallet Operations**
   - Create accounts
   - Transfer tokens
   - Check balances

---

## Documentation

### **How to Use the Wallet**
1. Build the project:
   ```bash
   cargo build --release
   ```

2. Run the wallet application:
   ```bash
   ./target/release/icp_wallet
   ```

3. Follow the on-screen prompts to perform wallet operations.

### **Directory Structure**
- `src/`: Contains the Rust source code.
- `tests/`: Includes unit and integration tests.
- `Cargo.toml`: Manages dependencies.
- `README.md`: Project documentation.

---

## Submission

1. Push your project to a GitHub repository.
   ```bash
   git add .
   git commit -m "Initial commit"
   git push origin main
   ```

2. Ensure the repository includes:
   - Source code
   - `README.md` with setup and usage instructions
   - A test report

3. Submit the GitHub repository link via the provided submission form.

---

## Contact
If you encounter issues or have questions, please reach out via the provided communication channel.

