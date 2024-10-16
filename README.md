# ℓ-Leveled Homomorphic Encryption in Rust

This Rust project implements the first ℓ-leveled homomorphic encryption (HE) schemes over composite groups, supporting both additive and multiplicative homomorphism. Built on the factoring problem, our design outperforms traditional lattice-based HE schemes in both efficiency and simplicity.

## Key Features
- **Additive & Multiplicative Homomorphism**: Perform secure operations on encrypted data without decryption.
- **No Relinearization**: Avoids the costly relinearization step seen in LWE-based schemes.
- **No Circular Security Assumption**: Simpler security model without sacrificing strength.
- **Double-Sized Message Support**: Encrypt messages up to twice the size without increasing ciphertext size.
- **Blazing Performance**: Up to 1000x faster in multiplication than BFV, BGV, CKKS, and TFHE.

## Installation

1. Install Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone the repo:
   ```bash
   git clone <repo-url>
   cd <repo-directory>
