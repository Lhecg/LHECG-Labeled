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
        // let d_l_file="src/data/d_l_values.txt";
        // let d_l_array= utility::read_values_from_file(d_l_file);

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
    fn keygen() {
        match config::load_config() {
            Ok(config) => {
                let params = calculate_parameters(config.lambda, config.t);
                let mut total_keygen_time = Duration::new(0, 0);
                let iterations = 2; // Number of iterations
            
                for number in 1..=iterations {                                    
                    let start_keygen = Instant::now();
        
                    // Prime generation using num-primes crate
                    let p_prime = Generator::new_prime(params.bits);
                    let p = BigUint::from_bytes_be(&p_prime.clone().to_bytes_be());
                    
                    let q_prime = Generator::new_prime(params.bits);
                    let q = BigUint::from_bytes_be(&q_prime.clone().to_bytes_be());
                    
                    // Calculate N = p * q
                    let N = &p * &q;
        
                    // Convert p to BigInt
                    let p_bigint = BigInt::from(p.clone());
        
                    // Random number d in range (0, p)
                    let mut rng = thread_rng();
                    let d = rng.gen_bigint_range(&BigInt::from(0), &p_bigint);
        
                    // Key generation timing
                    let keygen_time = start_keygen.elapsed();                    
                    println!("Keygen time: {:?}", keygen_time);
                    total_keygen_time += keygen_time; // Accumulate keygen time
                }
                let avg_keygen_time = total_keygen_time / iterations;            
                println!("Average keygen time: {:?}", avg_keygen_time);
            }
            Err(e) => eprintln!("Failed to load config: {}", e),
        }
    
        // Assertion
        assert_eq!(1, 1);
    }



    #[test]
    fn enc_dec_timing() {
        let d_l_file="src/data/d_l_values.txt";
        let d_l_array= utility::read_values_from_file(d_l_file);
        println!("===============================================");
        println!("Evaluating the encryption and decryption timing");
        println!("===============================================");
        let config = config::load_config().expect("Failed to load config");
        let setup = setup();
        let mut rnd = rand::thread_rng();
    
        // Variables to store the total encryption and decryption times
        let mut total_encryption_time = Duration::new(0, 0);
        let mut total_decryption_time = Duration::new(0, 0);
    
        let iterations = 10; // Number of iterations
    
        for number in 1..=iterations {
            let m1 = rnd.gen_range(0..config.t - 1);
            println!("m1 from range z_t: {:?}", m1);
            
            let start_enc = Instant::now();               
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
            let encryption_time = start_enc.elapsed();
            total_encryption_time += encryption_time; // Accumulate encryption time
            println!("Encryption time: {:?}", encryption_time);
    
            let start_dec = Instant::now();
            decryption::decrypt_method_one(&c1, &setup.p, &setup.q, &d_l_array, &setup.bigint_t, l1);
            let decryption_time = start_dec.elapsed();
            total_decryption_time += decryption_time; // Accumulate decryption time
            println!("Decryption time: {:?}", decryption_time);
            println!("--------------------------");
        }
        let avg_encryption_time = total_encryption_time / iterations;
        println!("Average encryption time: {:?}", avg_encryption_time);
        let avg_decryption_time = total_decryption_time / iterations;
        println!("Average decryption time: {:?}", avg_decryption_time);
    }

    /* 
    #[test]
    fn enc_dec_timing_constant_mi() {
        println!("===================================================================");
        println!("Evaluating the encryption and decryption timing while m is constant");
        println!("===================================================================");
        let config = config::load_config().expect("Failed to load config");
        let setup = setup();
        let mut rnd = rand::thread_rng();
    
        // Variables to store the total encryption and decryption times
        let mut total_encryption_time = Duration::new(0, 0);
        let mut total_decryption_time = Duration::new(0, 0);
    
        let iterations = 100; // Number of iterations

        let m1 = rnd.gen_range(0..config.t - 1);
        println!("m1 from range z_t: {:?}", m1);
        for number in 1..=iterations {            
            let start_enc = Instant::now();               
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
            let encryption_time = start_enc.elapsed();
            total_encryption_time += encryption_time; // Accumulate encryption time
            println!("Encryption time: {:?}", encryption_time);
    
            let start_dec = Instant::now();
            decryption::decrypt_method_one(&c1, &setup.p, &setup.q, &setup.d, &setup.bigint_t, l1);
            let decryption_time = start_dec.elapsed();
            total_decryption_time += decryption_time; // Accumulate decryption time
            println!("Decryption time: {:?}", decryption_time);
            println!("--------------------------");
        }        
    
        // Compute the average times
        let avg_encryption_time = total_encryption_time / iterations;
        let avg_decryption_time = total_decryption_time / iterations;
    
        println!("Average encryption time: {:?}", avg_encryption_time);
        println!("Average decryption time: {:?}", avg_decryption_time);
    
        assert_eq!(1, 1);
    }

*/
    #[test]
    fn add_mult_timing() {
        let d_l_file="src/data/d_l_values.txt";
        let d_l_array= utility::read_values_from_file(d_l_file);
        println!("====================================");
        println!("Testing the add_mult_timing function");
        println!("====================================");
        let config = config::load_config().expect("Failed to load config");
        let setup = setup();
        let mut rnd = rand::thread_rng();
    
        // Variables to store the total encryption and decryption times
        let mut total_add_time = Duration::new(0, 0);
        let mut total_mult_time = Duration::new(0, 0);
    
        let iterations = 10; // Number of iterations
    
        for number in 1..=iterations {
            let m1 = rnd.gen_range(0..config.t - 1);
            println!("m1 from range z_t: {:?}", m1);                                    
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

            let m2 = rnd.gen_range(0..config.t - 1);
            println!("m2 from range z_t: {:?}", m2);
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

            let start_add = Instant::now();
            let (c_add, l_add) = homomorphic::homomorphic_add(&c1, &c2, &setup.N, l1);
            let add_time = start_add.elapsed();
            total_add_time += add_time; // Accumulate encryption time
            println!("Add time: {:?}", add_time);

            let start_mult = Instant::now();
            let (c_mult, l_mult) = homomorphic::homomorphic_mult(&c1, &c2, &setup.N, l1,l2);
            let mult_time = start_mult.elapsed();
            total_mult_time += mult_time; // Accumulate mult time
            println!("Mult time: {:?}", mult_time);
            println!("--------------------------");                                                            
        }
    
        // Compute the average times
        let avg_add_time = total_add_time / iterations;
        let avg_mult_time = total_mult_time / iterations;
    
        println!("Average add time: {:?}", avg_add_time);
        println!("Average mult time: {:?}", avg_mult_time);
    
        assert_eq!(1, 1);
    }

/* 
    #[test]
    fn add_mult_timing_constant_mi() {
        println!("====================================");
        println!("Testing the add_mult_timing function");
        println!("====================================");
        let config = config::load_config().expect("Failed to load config");
        let setup = setup();
        let mut rnd = rand::thread_rng();
    
        // Variables to store the total encryption and decryption times
        let mut total_add_time = Duration::new(0, 0);
        let mut total_mult_time = Duration::new(0, 0);
    
        let m1 = rnd.gen_range(0..config.t - 1);
        println!("m1 from range z_t: {:?}", m1);
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
        let m2 = rnd.gen_range(0..config.t - 1);
        println!("m2 from range z_t: {:?}", m2);
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

        let iterations = 1000; // Number of iterations
        for number in 1..=iterations {                                                
            let start_add = Instant::now();
            let (c_add, l_add) = homomorphic::homomorphic_add(&c1, &c2, &setup.N, l1);
            let add_time = start_add.elapsed();
            total_add_time += add_time; // Accumulate encryption time
            println!("Add time: {:?}", add_time);

            let start_mult = Instant::now();
            let (c_mult, l_mult) = homomorphic::homomorphic_mult(&c1, &c2, &setup.N, l1,l2);
            let mult_time = start_mult.elapsed();
            total_mult_time += mult_time; // Accumulate mult time
            println!("Mult time: {:?}", mult_time);
            println!("--------------------------");                                                            
        }
    
        // Compute the average times
        let avg_add_time = total_add_time / iterations;
        let avg_mult_time = total_mult_time / iterations;
    
        println!("Average add time: {:?}", avg_add_time);
        println!("Average mult time: {:?}", avg_mult_time);
    
        assert_eq!(1, 1);
    }
    */

}