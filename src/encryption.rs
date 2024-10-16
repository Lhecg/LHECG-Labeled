use num_bigint::BigUint;
use num_bigint::{BigInt, RandBigInt};
use num_traits::Pow;
use rand::{thread_rng, Rng};
// use num_bigint::BigUint;
use num_traits::One;

pub fn encrypt(
    message: &BigInt,
    t:&BigInt,
    rhu:&usize,
    p: &BigUint,
    q:&BigUint,
    D:&BigInt,
    N: &BigUint,
    bits:usize
) -> (BigInt , usize){

   
    let p = BigInt::from(p.clone());
    let q = BigInt::from(q.clone());

    let N = BigInt::from(N.clone());

let label=1;
    
let mut r=generate_random_range(&q);
let e=randint(*rhu); //todo 
let e = BigInt::from(e.clone());
// let d = generate_random_range(&p);
    let mu = (t * &e) +message;
    let ciphertext = (D*mu + p * r) % N;
   ( ciphertext , label)
    // d randomly choosen from z_p
    //  d q* mu + p * r % N;
    // dq((te +message))+pr 
    // mod p --> dq(te+message) 
    // dq(te+message) mod t --> dqm 
    // 
}
fn randint(bits: usize) -> BigInt {
    let mut rng = thread_rng();
    let mut x = BigInt::default();
    for _ in 0..bits {
        x <<= 1;
        x += rng.gen_range(0..=1);
    }
    x
}

fn generate_random_range(n: &BigInt) -> BigInt {
    let mut rng = thread_rng();
    rng.gen_bigint_range(&BigInt::from(0), n)
}
