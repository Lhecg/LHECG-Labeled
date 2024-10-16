
use num_bigint::BigInt;
use num_bigint::BigUint;

pub fn homomorphic_add(ciphertext1: &BigInt, ciphertext2: &BigInt, modulus_n: &BigUint, l:usize) -> (BigInt, usize) {
    let modulus_n = BigInt::from(modulus_n.clone());
    ((ciphertext1 + ciphertext2) % modulus_n, l)
}

pub fn homomorphic_mult(ciphertext1: &BigInt, ciphertext2: &BigInt, modulus_n: &BigUint, label1:usize, label2:usize) -> ( BigInt, usize) {
    let modulus_n = BigInt::from(modulus_n.clone());
   ( (ciphertext1 * ciphertext2) % modulus_n, label1+label2)
}

pub fn homomorphic_sub(ciphertext1: &BigInt, ciphertext2: &BigInt, modulus_n: &BigUint) -> BigInt {
    let modulus_n = BigInt::from(modulus_n.clone());
    (ciphertext1 - ciphertext2) % modulus_n
}