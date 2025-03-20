ICP TOKEN WALLET :

Overview :
ICP Token Wallet is a **Rust-based smart contract** for managing **ICRC-2 tokens** on the Internet Computer (ICP) blockchain. This wallet enables **secure token transfers, balance checks, and transaction approvals** using smart contracts.  

---

## **Features**  
✅ Send ICRC-2 tokens  
✅ Receive tokens & update balances  
✅ Approve & track token spending  
✅ Retrieve transaction history  
✅ Fully tested using Rust unit tests  

---

## **Setup & Installation**  

### **Prerequisites**  
Ensure the following dependencies are installed:  
- **Rust & Cargo** → Install via [Rustup](https://rustup.rs/):  
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **DFX SDK (Internet Computer)** → Install via:  
  ```sh
  sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
  ```
- **Node.js & NPM (for frontend, if needed)**  
  ```sh
  sudo apt install nodejs npm -y
  ```

---

## **Getting Started**  

### **Clone the Repository**  
```sh
git clone <https://github.com/shivaraaman/QuadBtech_ICRC_token>
cd token_transfer_from_backend
```

### **Start Local ICP Network**  
```sh
dfx start --background --clean
```

### **Deploy Smart Contracts**  
```sh
dfx deploy icrc1_ledger_canister --argument "(variant {
  Init = record {
    token_symbol = \"ICRC1\";
    token_name = \"L-ICRC1\";
    minting_account = record {
      owner = principal \"$(dfx identity --identity anonymous get-principal)\"
    };
    transfer_fee = 10_000;
    metadata = vec {};
    initial_balances = vec {
      record {
        record {
          owner = principal \"$(dfx identity --identity default get-principal)\";
        };
        10_000_000_000;
      };
    };
    archive_options = record {
      num_blocks_to_archive = 1000;
      trigger_threshold = 2000;
      controller_id = principal \"$(dfx identity --identity anonymous get-principal)\";
    };
    feature_flags = opt record {
      icrc2 = true;
    };
  }
})"
```

### **Check Canister Status**  
```sh
dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
  owner = principal \"$(dfx identity --identity default get-principal)\";
})"
```

---

### **Deploy the token transfer cansiter**  
```sh
dfx deploy token_transfer_from_backend
```

## **Usage**  

### **1️⃣ Check Token Balance**  
```sh
dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
  owner = principal \"$(dfx identity --identity default get-principal)\";
})"
```

### **2️⃣ Approve Token Spending**  
```sh
dfx canister call --identity default icrc1_ledger_canister icrc2_approve "(
  record {
    spender= record {
      owner = principal \"$(dfx canister id token_transfer_from_backend)\";
    };
    amount = 10_000_000_000: nat;
  }
)"
```

### **3️⃣ Transfer Tokens**  
```sh
dfx canister call token_transfer_from_backend transfer "(record {
  amount = 100_000_000;
  to_account = record {
    owner = principal \"$(dfx canister id token_transfer_from_backend)\";
  };
})"
```

### **4️⃣ Fetch Transaction History**  
```sh
dfx canister call icrc1_ledger_canister get_transactions "(record {
  account = record { owner = principal \"$(dfx identity --identity default get-principal)\" };
  start = 0 : nat;
  length = 10 : nat;
})"
```

---

## **Running Tests**  
To run Rust unit tests, execute:  
```sh
cargo test --test token_tests
```

---

## **Future Enhancements**  
🚀 **Web-based UI Integration**  
🚀 **Multi-user Authentication**  
🚀 **Optimized Transaction Fees**  
🚀 **Deployment on ICP Mainnet**  

---

## **License**  
This project is **open-source** under the **MIT License**.  

---
