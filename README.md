# Leveled Homomorphic Encryption over
Composite Groups

This repository contains the implementation of the first ℓ-leveled homomorphic encryption (HE) schemes over composite groups, offering both multiplicative and additive homomorphic properties. Unlike traditional Fully Homomorphic Encryption (FHE) schemes based on lattice-based or AGCD-based approaches, our scheme is based on the factoring problem and achieves significant improvements in performance.
Key Features:
Additive and Multiplicative Homomorphism: Supports both addition and multiplication on encrypted data, addressing the limitations of traditional Partial Homomorphic Encryption (PHE) schemes.
No Relinearization: Our design eliminates the need for the relinearization operation commonly required in LWE-based HE schemes.
No Circular Security Assumption: Removes the circular security assumption, further simplifying the security model.
Expanded Message Space: Supports an extended message space, and with the introduction of the "Double-Sized Message" technique, it can encrypt messages up to twice the size without increasing ciphertext size.
Superior Performance: Benchmarks show our schemes outperform well-known HE schemes like BFV, BGV, CKKS, and TFHE, especially in multiplication operations, with speeds up to 1000 times faster.
Motivation:
Homomorphic encryption allows computations to be performed on encrypted data without decrypting it, which is essential for secure cloud computing, privacy-preserving machine learning, and secure data analysis. Our approach provides a solution that combines both homomorphic properties over composite groups, achieving efficiency and scalability that surpass existing solutions.

Installation:
Instructions for installing and running the project.

Usage:
   ```bash
   git clone <repository-url>
   cd <repository-directory>
   cargo run --release


Benchmarks:
cargo test --release -- --nocapture

Contributing:
Information on how to contribute to this project.

License:
Details of the project’s license.




