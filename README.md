# HD-50: Portable Secure Vault

**A Rust-based, Host-Executed Data Vault with Self-Destruct Capabilities.**

This project turns any USB flash drive into a secure, portable digital safe that runs directly on Windows without installation. It features an active defense mechanism that destroys the data after 3 failed password attempts.

## Features
- **Zero-Installation**: Runs directly from the USB (`HD50.exe`).
- **AES-256-GCM Encryption**: Uses industry-standard, authenticated encryption.
- **Argon2 Key Derivation**: Protects against brute-force attacks.
- **Active Defense (Wipe)**: If an attacker guesses the password wrong 3 times, the encrypted container (`VAULT.DAT`) is securely shredded (overwritten with 0s) and deleted.
- **Host-Based TUI**: Provides a clean, professional Command Line Interface (CLI) on the host computer.

## Requirements
- Rust Toolchain (for building).
- Windows 10/11 (Host OS).

## Usage
1. **Deployment**: Copy `HD50.exe` and `autorun.inf` to the USB root.
2. **Locking**:
   - Create a folder `UNLOCKED_CONTENTS` on the USB.
   - Run `HD50.exe`.
   - Set a password to encrypt and shred the folder.
3. **Unlocking**:
   - Run `HD50.exe`.
   - Enter password.
   - Files are extracted to `UNLOCKED_CONTENTS`.

## Build
```bash
python build_vault.py
```
