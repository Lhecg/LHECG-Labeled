use std::fs::File;
use std::io::{self, Write};
use num::{integer::Integer, One};
  use num_bigint::BigUint;
  use num::bigint::BigInt;
  

//   use num_bigint_dig::algorithms::mod_inverse;

pub fn save_encrypted_values(c1: &BigInt, N: &BigInt, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for i in 1..=10 {
        let ci = c1.modpow(&BigInt::from(i), N); // c1^i mod N
        writeln!(file,  "{}", ci);
    }
Ok(())
}


pub fn generate_and_save_d_l(D: &BigInt, p: &BigInt,   filename: &str)-> io::Result<()> {
    // let file = File::create("D_l_values.txt").expect("Unable to create file");
    let mut file = File::create(filename)?;

     let D_inv= mod_inverse(&D,&p);
  //  let D_inv= mod_inverse(&D,&p);

    // Generate and save D_l for l in range 0 to l
    for k in 0..=10 {
        let D_l = &D_inv.pow(k as u32) % p; //  
        writeln!(file,  "{}", D_l);
    }
    
    Ok(())
}


fn mod_inverse(a: &BigInt, m: &BigInt) -> BigInt {
    let gcd_result = a.extended_gcd(m); 
    let (g, x, _y) = (gcd_result.gcd, gcd_result.x, gcd_result.y); 

    if g == BigInt::from(1) {
        (x % m + m) % m 
    } else {
        BigInt::from(0) 
    }
}
    // let mut file = File::create("encrypted_values.txt")?;

  
