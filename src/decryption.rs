
/// Decrypts the ciphertext c using the secret key p and q
use num::{integer::Integer, One};
  use num_bigint::BigUint;
  use num::bigint::BigInt;
//   use modinverse::modinverse;

  // use num::bigint;

  pub fn decrypt_method_one(ciphertext: &BigInt, p: &BigUint,q: &BigUint,d_l_array:&[BigInt;10], t: &BigInt, l:usize) -> BigInt {
    let p = BigInt::from(p.clone());
    // let q = BigInt::from(q.clone());
    // let N=&p*&q;
    // let dq=d*&q; // preproccess
  //  let D_inv= mod_inverse(&D,&p);
  //  let D_l=&D_inv.pow(l as u32);
  let D_l=&d_l_array[l];
     let mu=ciphertext*D_l % &p;  // cb=dmu
        let message = mu % t; 
    message
  }
  
pub fn decrypt_method_two(ciphertext: &BigInt, p: &BigUint,q: &BigUint,d:&BigInt, t: &BigInt, l:i32) -> BigInt {
    let p = BigInt::from(p.clone());
    let q = BigInt::from(q.clone());
    let N=&p*&q;
   let d_inv= mod_inverse(&d,&N);
   let d_l=&d_inv.pow(l as u32);

     let gcd_result = p.extended_gcd(&q);

     let b: BigInt = gcd_result.y;

let q_power_p=q.modpow(&(p-BigInt::from(l)), &N);

     let cb=ciphertext*d_l*q_power_p % &N;  // cb=dmu
        let mu=cb.div_ceil(&q);
        let message = mu % t; 
    message
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

// ap+bq=1 ---> acp+bcq=c 
//               pr+dmq=c ==> ac=r ,bc=dmq  