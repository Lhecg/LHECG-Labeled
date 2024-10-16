use num_bigint::RandBigInt;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_primes::Generator;
use num_traits::{One, Zero};
use rand::{thread_rng, Rng};
use std::str::FromStr;
use std::time::{Instant, Duration};
use Fhscheme_Labeled::config;
use Fhscheme_Labeled::decryption;
use Fhscheme_Labeled::encryption;
use Fhscheme_Labeled::equalEvalPreprocess::{Polynomial as EqPPolynomial, Term as EqPTerm};
use Fhscheme_Labeled::equalizedevaluation::{Polynomial as EqPolynomial, Term as EqTerm};
use Fhscheme_Labeled::equalizer::equalizer;
use Fhscheme_Labeled::evaluation::{Polynomial, Term};
use Fhscheme_Labeled::homomorphic;
//use Fhscheme_Labeled::multiThreadevaluation::{Polynomial as Multipoly, Term as Multiterm};
use Fhscheme_Labeled::parameters::calculate_parameters;
use Fhscheme_Labeled::parameters::Parameters;
use Fhscheme_Labeled::utility;
use num_traits::ToPrimitive;
use num::{integer::Integer};

#[cfg(test)]  //Ehsan: we do not need this.
mod tests {  //Ehsan: we do not need this.

    use toml::value::Array;

    use super::*;  //Ehsan: we do not need this.

    struct TestSetup {
        p: BigUint,
        q: BigUint,
        d: BigInt,
        N: BigUint,
        bigint_t: BigInt,
        params: Parameters,
    }

    fn setup() -> TestSetup {
        let config = config::load_config().expect("Failed to load config");
        let params = calculate_parameters(config.lambda, config.t);
        // let p_prime = Generator::new_prime(params.bits);
        // let p_byte = p_prime.clone().to_bytes_be();
        // let p = BigUint::from_bytes_be(&p_byte);
        // let q_prime = Generator::new_prime(params.bits);
        // let q_byte = q_prime.clone().to_bytes_be();
        // let q = BigUint::from_bytes_be(&q_byte);
        let bigint_t = BigInt::from(config.t);
        println!("The t value is :{:?}", bigint_t);

        let param_file = "src/keypair/prime_number.txt";
 



        // Check if parameters are already saved
        let (p, q,d, N) = if let Ok((p, q,d, N)) = utility::load_parameters_from_file(param_file) {
            (p, q,d, N)
        } else {
            println!("Generating and saving new parameters...");
            let p_prime = Generator::new_prime(params.bits); // use num-primes crate
            let p_byte = p_prime.clone().to_bytes_be();
            let p = BigUint::from_bytes_be(&p_byte);

            let q_prime = Generator::new_prime(params.bits); // use num-primes crate
            let q_byte = q_prime.clone().to_bytes_be();
            let q = BigUint::from_bytes_be(&q_byte);

            let N = &p * &q;
            let _p=BigInt::from(p.clone());
            let mut rng = thread_rng();
            
            let d=   rng.gen_bigint_range(&BigInt::from(0),&_p);
            // Save parameters to file
            utility::save_parameters_to_file(param_file, &p, &q,&d, &N)
                .expect("Failed to save parameters to file");
            (p, q,d, N)
        };

        TestSetup {
            p,
            q,
            d,
            N,
            bigint_t,
            params,
        }
    }


    #[test]
    fn test_lemm2(){
        let setup = setup();
        let p_bigint=BigInt::from(setup.p.clone());
        let q_bigint=BigInt::from(setup.q.clone());
        let n_bigint=BigInt::from(setup.N);
        // let q_bigint=BigInt::from(clone.setup.q);
        let p_hat=&p_bigint.modpow(&( &q_bigint- BigInt::from(1)), &n_bigint);
        let q_hat=&q_bigint.modpow(&( &p_bigint- BigInt::from(1)), &n_bigint);
        // let gcd_result = p.extended_gcd(&q);
        //   p_hat.extended_gcd(&setup.N);
        assert_eq!(p_hat.modpow(&BigInt::from(2),&n_bigint ),p_hat.clone());
        assert_eq!(p_hat.gcd(&n_bigint), p_bigint);
        assert_eq!((p_hat-BigInt::from(1)).gcd(&n_bigint), q_bigint);
        assert_eq!(q_hat.modpow(&BigInt::from(2),&n_bigint ),q_hat.clone(), "q_hat ^2=q_hat");
        assert_eq!(q_hat.gcd(&n_bigint), q_bigint);
        assert_eq!((q_hat-BigInt::from(1)).gcd(&n_bigint), p_bigint);
        // ap+bq=1 
    }
    

    #[test]
    fn enc_dec_correctness () {
        let d_l_file="src/data/d_l_values.txt";
        let d_l_array= utility::read_values_from_file(d_l_file);
        println!("======================================================");
        println!("Testing the encryption_decryption_correctness function");
        println!("======================================================");        
        let config = config::load_config().expect("Failed to load config");
        let setup = setup();
        let mut rnd = rand::thread_rng();
        let m1 = rnd.gen_range(0..config.t - 1);
        println!("m1 from range z_t:{:?}", m1);
        let (c1, l1) = encryption::encrypt(
            &BigInt::from(m1),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );   
  
        println!("c1, as the encryption of m1, equals {:?}", c1);
        let decrypted_c1 = decryption::decrypt_method_one(&c1, &setup.p, &setup.q, &d_l_array, &setup.bigint_t, l1);
        println!("checking the corectness of dectption ====> message is :{}, decyption result is:{}", m1,  decrypted_c1);
        // to test equality of m1 and decrypted_c1, type of decrypted_c1 should changes from bitint into usize, unless assert_eq! will fail.
        let decrypted_c1_u64 = decrypted_c1.to_u64().expect("Decrypted value is too large to fit into u64");
        let decrypted_c1_usize = decrypted_c1_u64 as usize;
        assert_eq!(m1 as usize, decrypted_c1_usize);
    }

    #[test]
    fn add_mult_correctness() {
        let d_l_file="src/data/d_l_values.txt";
        let d_l_array= utility::read_values_from_file(d_l_file);
        println!("========================================");
        println!("Testing the add_mult_correction function");
        println!("========================================");
        let config = config::load_config().expect("Failed to load config");
        let setup = setup();

        let mut random_msg=rand::thread_rng();
        let m1: usize =random_msg.gen_range(0..=config.t-1);                    
        let (c1, l1) = encryption::encrypt(
            &BigInt::from(m1),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        ); 
        let m2 =random_msg.gen_range(0..=config.t-1);                      
        let (c2, l2) = encryption::encrypt(
            &BigInt::from(m2),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );
        // let hommorphic_add_time = start_hom.elapsed();
        let (c_add, l_add) = homomorphic::homomorphic_add(&c1, &c2, &setup.N, l1);           
        // println!("hommorphic addition time: {:?}", hommorphic_add_time);
        let decypted_add = decryption::decrypt_method_one(&c_add, &setup.p, &setup.q, &d_l_array, &setup.bigint_t, l_add); 

        let (c_mult, l_mult) = homomorphic::homomorphic_mult(&c1, &c2, &setup.N, l1,l2);
        let decypted_mult=decryption::decrypt_method_one(&c_mult, &setup.p, &setup.q, &d_l_array, &setup.bigint_t, l_mult);
        println!("m1 :{}, m2 :{} ==>", m1, m2);
        println!("m1+m2:{},   and Dec(c1+c2):{}", (m1+m2 %&setup.bigint_t), decypted_add);
        println!("m1*m2:{},   and Dec(c1*c2):{}", (m1*m2 %&setup.bigint_t), decypted_mult);
        println!("Consider that computations are mode t, which is:{}\n", &setup.bigint_t);
    }

}