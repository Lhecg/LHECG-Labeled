use num_bigint::{BigInt, BigUint,ToBigInt};
use num_bigint::{ RandBigInt};
use num_traits::{One, Zero};
use Fhscheme_Labeled::encryption::encrypt;
use std::str::FromStr;
use rand::{thread_rng, Rng};
use Fhscheme_Labeled::encryption;
use Fhscheme_Labeled::decryption;
use Fhscheme_Labeled::config;
use Fhscheme_Labeled::parameters::calculate_parameters;
use Fhscheme_Labeled::utility;
use num_primes::Generator;
use Fhscheme_Labeled::homomorphic;
use std::time::Instant;
use Fhscheme_Labeled::evaluation;
use Fhscheme_Labeled::equalizer;
use crate::equalizer::equalizer;
use std::path::Path;
use Fhscheme_Labeled::preprocessing;
use std::collections::HashMap;
use Fhscheme_Labeled::evaluation::{Polynomial,Term};  // Import the Polynomial module
// use num_bigint::BigUint;

fn main() {

    match config::load_config() {
        Ok(config) => {
            let params = calculate_parameters(config.lambda, config.t);
            let bigint_t = BigInt::from(config.t);   //Ehsan: t is not a bigint.
            
            // Reading (generating) parameters p, q, N, and d.
            // let l:usize=10;
            let d_l_file="src/data/d_l_values.txt";
            let encrypted_one_file = "src/data/preprocessed_encrypted_data.txt";
            let param_file = "src/keypair/prime_number.txt";
            // Check if parameters are already saved
            let (p, q,D, N) = if let Ok((p, q,D, N)) = utility::load_parameters_from_file(param_file) {
                println!("Parameters loaded from file.");
                (p, q, D,N)
            } else {
                println!("Generating and saving new parameters...");
                let start_KeyGen = Instant::now();
                let p_prime = Generator::new_prime(params.bits); // use num-primes crate
                let p_byte = p_prime.clone().to_bytes_be();
                let p = BigUint::from_bytes_be(&p_byte);
                let q_prime = Generator::new_prime(params.bits); // use num-primes crate
                let q_byte = q_prime.clone().to_bytes_be();
                let q = BigUint::from_bytes_be(&q_byte);
                let N = &p * &q;
                let _p=BigInt::from(p.clone());
                let mut rng = thread_rng();
                let D=   rng.gen_bigint_range(&BigInt::from(1),&(_p - BigInt::from(1)));
                let KeyGen_time = start_KeyGen.elapsed();
                println!("KeyGen time: {:?}", KeyGen_time);
                // Save parameters to file
                utility::save_parameters_to_file(param_file, &p, &q,&D, &N).expect("Failed to save parameters to file");
                (p, q,D, N)

             

            };  
            println!("Generating and saving d_l values and encyption of one...");

            preprocessing::generate_and_save_d_l(&D,&BigInt::from(p.clone()), d_l_file).expect("Failed to save d_l values to file");

            let modulus_n=BigInt::from(N.clone());
            let (one_encrypted, l_1) = encryption::encrypt(&BigInt::from(1), &bigint_t, &params.rhu, &p, &q,&D, &N, params.bits);

            preprocessing::save_encrypted_values(&one_encrypted, &modulus_n, encrypted_one_file);       

            //  if p, q change, then this file should be updated too.
    

            // println!("decrypt one:{}", decryption::decrypt_method_one(&one_encrypted, &p, &q, &D, &bigint_t, l_1));
            // if Path::new(filename).exists() {
            //     println!("File '{}' already exists.", filename);
            // } else {

            println!("loading d_l values from file and passing to decryption method as an array");
           let d_l_array= utility::read_values_from_file(d_l_file);
                let (one_encrypted, l_1) = encryption::encrypt(&BigInt::from(1), &bigint_t, &params.rhu, &p, &q,&D, &N, params.bits);
                println!("decrypt one:{}", decryption::decrypt_method_one(&one_encrypted, &p, &q,&d_l_array, &bigint_t, l_1));
                
        /*
        let polynomial = Polynomial::new(terms);
        let variables = [
            ("c1".to_string(), c1),
            ("c2".to_string(), c2)
        ].iter().cloned().collect();
        let result = polynomial.evaluate(&variables);
        println!("The result is: {}", result);
        */
    
        /*
        // c1, eval( c1+c2) , eval(c1*c2) ,eval( f(x)= c1+c1c2+c1^3; f=c1c2c3+c1c4+c5
        let one_enc = encryption::encrypt(&BigInt::from(1), &bigint_t, &params.rhu, &p, &q, &N, params.bits);
        let start_equalizer = Instant::now();
        let poly = "( c1  * c2 * c4) +  ( c3 * c1 ) + ( c2 )";
        let equalized_poly = equalizer("( c1  * c2 * c4) +  ( c3 * c1 ) + ( c2 )");
        let end_equalizer_time = start_equalizer.elapsed();
        println!("equalized output: {}", equalized_poly);
        println!("equalizer_time: {:?}", end_equalizer_time);
        let one_enc = BigInt::from(1);
        let start_evaluation=Instant::now();
        */


        /*    
        let end_evaluation_time=start_evaluation.elapsed();
        println!("evaluation time:{:?}",end_evaluation_time);
        println!("poly output:{:?}",output);
        print!("equalized:{}",equalizer("( c1  * c2) + (c3)"));
        */

        /*
        let ciphetexts = vec![
        BigInt::from(2), // c1
        BigInt::from(3), // c2
        BigInt::from(5), // c3
        BigInt::from(7), // c4
        ];
        let output= evaluation::eval(ciphetexts, 10, &equalized_poly,&one_enc );
        let output= evaluation::eval(ciphetexts, 10, "( c1  * c2 * c4) +  ( c3 * c1 ) + ( c2 )",&one_enc );
        */

        },

        Err(e) => eprintln!("Failed to load config: {}", e),
    }    
}
