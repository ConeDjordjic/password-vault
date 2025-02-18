# 🔒 PassVault, a Hardened Password Manager

A CLI-based password manager with AES-256-GCM encryption written in Rust for Cybersecurity and Cryptography practice and to learn Rust!

![License](https://img.shields.io/badge/license-MIT-blue)

## Features
- AES-256-GCM encryption
- Argon2 key derivation
- Secure password generation
- Encrypted local storage
- CLI interface

## Installation
```bash
# Prerequisites
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone & Build
git clone https://github.com/ConeDjordjic/password-vault.git
cd password-vault
cargo install --path .
```

## Usage
```bash
# Add credentials
passvault add github.com your@email.com

# Retrieve credentials
passvault get github.com

# Generate password
passvault generate --length 24 --special
```

## Security
- Encrypted vault format
- Memory-safe implementation
- No master password storage
- Regular security audits
