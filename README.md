# 🦀 Rust Backend (Axum + Tokio + Rust)

A lightweight **Rust backend server** built with **Axum** and **Tokio**, exposing Solana-related APIs.  
This backend provides endpoints to **generate keypairs, sign/verify messages, transfer SOL, and handle SPL tokens**.

---

## 🚀 Overview

This project simplifies Solana development by providing ready-to-use HTTP endpoints that abstract away complex Solana instruction building.  
It is stateless, secure, and easily extendable for new Solana features like Token-2022.

---

## 🔑 Endpoints

### 1. Generate Keypair
- **POST /keypair**
- Generates a new Solana keypair (public + secret).

### 2. Create Token Mint
- **POST /token/create**
- Creates a new SPL Token mint with a given authority and decimals.

### 3. Mint Tokens
- **POST /token/mint**
- Mints new tokens to a destination account.

### 4. Sign Message
- **POST /message/sign**
- Signs a message using a provided secret key.

### 5. Verify Message
- **POST /message/verify**
- Verifies that a signature belongs to a given message and public key.

### 6. Send SOL
- **POST /send/sol**
- Creates a native SOL transfer instruction between two accounts.

### 7. Send Token
- **POST /send/token**
- Creates an SPL Token transfer instruction between two accounts.

---

## ⚙️ Technical Details

- Uses **Ed25519** for message signing and verification.
- Keys encoded in **Base58**.
- Signatures and instruction data encoded in **Base64**.
- Structured error handling for safer responses.
- Stateless design (no private key storage).

---

## 🔐 Security Considerations

- 🚫 No private keys are stored on the server.
- ✅ Input validation for all endpoints.
- ✅ Uses secure cryptographic primitives from Solana crates.
- ✅ Proper error handling with no sensitive info leakage.

---

## 🛠️ Tech Stack

- [Axum](https://github.com/tokio-rs/axum) – Web framework  
- [Tokio](https://tokio.rs) – Async runtime  
- [Solana SDK](https://docs.rs/solana-sdk) – Solana primitives  
- [SPL Token](https://spl.solana.com/token) – Token program support  

---

## 📄 License

MIT License © 2025
