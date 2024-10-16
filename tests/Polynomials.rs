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

    /*
    #[test]
    #[ignore]
    fn simple_addition (){
        let config = config::load_config().expect("Failed to load config");
        let setup = setup();
        let mut rnd = rand::thread_rng();
        let m1 = rnd.gen_range(0..config.t - 1);
        println!("m1 from range z_t:{:?}", m1);
        let m2 = rnd.gen_range(0..config.t - 1);
        println!("m2 from range z_t:{:?}", m2);

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

         
        let modulus_n = BigInt::from(setup.N.clone());
        // let f_test = "c1 * c2 * c3 * c4 * c5  + 7 c6 * c7 +   5 c8 * c9 * c10";

        let terms = vec![
            EqPTerm {
                coefficient: BigInt::one(), // No explicit coefficient, so it is 1
                variables: [
                    ("c1".to_string(), 1),
                    ("c2".to_string(), 1),
                ]
                .iter()
                .cloned()
                .collect(),
            }
        ];

        let start_eval_equalized = Instant::now();
        let polynomial: EqPPolynomial = EqPPolynomial::new(terms);
        // Define variable values
        
        let variables = [
            ("c1".to_string(), c1.clone()),
            ("c2".to_string(), c2.clone()),
        ]
        .iter()
        .cloned()
        .collect();
        /*
        let (degree_differences ,max_degree) = polynomial.find_degree_differences();
        let array=Fhscheme_Labeled::utility::read_values_from_file("src/data/preprocessed_encrypted_data.txt");
        //println!("max degree:{:?}- degree diffrence:{:?}", max_degree,degree_differences);
        let eval_res = polynomial.evaluate(&variables, &degree_differences, &array ,&modulus_n);
        let decrypted_eval_res = decryption::decrypt_method_one(&eval_res, &setup.p, &setup.q, &setup.d, &setup.bigint_t, max_degree);
        println!("decrypted_eval_res is :{:?}", decrypted_eval_res);


        let manual_res= &(c1 + c2) % modulus_n;
        //println!("manual :{:?}",manual_res);
        let expected_result=(m1 + m2)% &setup.bigint_t;
        println!("expected_result is {:?}", expected_result);

        // println!("check test eval 5 vari");
        // assert_eq!(expected_result,decryption::decrypt_method_one(&manual_res, &setup.p, &setup.q, &setup.d, &setup.bigint_t, 5));
        assert_eq!(expected_result, decrypted_eval_res);
        */
    }

    */
    
    #[test]
    fn ten_variables_function_correctness() {
        let d_l_file="src/data/d_l_values.txt";
        let d_l_array= utility::read_values_from_file(d_l_file);
        println!("==============================================================");
        println!("Testing the evaluation of c1*c2*c3*c4*c5 + 7c6*c7 + 5c8*c9*c10");
        println!("==============================================================");
        let config = config::load_config().expect("Failed to load config");

        let setup = setup();

        //println!(" p is : {:?}",setup.p);
        let mut rnd = rand::thread_rng();

        let m1 = rnd.gen_range(0..config.t - 1);
        println!("m1 from range z_t:{:?}", m1);
        let m2 = rnd.gen_range(0..config.t - 1);
        println!("m2 from range z_t:{:?}", m2);
        let m3 = rnd.gen_range(0..config.t - 1);
        println!("m3 from range z_t:{:?}", m3);
        let m4 = rnd.gen_range(0..config.t - 1);
        println!("m4 from range z_t:{:?}", m4);
        let m5 = rnd.gen_range(0..config.t - 1);
        println!("m5 from range z_t:{:?}", m5);
        let m6 = rnd.gen_range(0..config.t - 1);
        println!("m6 from range z_t:{:?}", m6);
        let m7 = rnd.gen_range(0..config.t - 1);
        println!("m7 from range z_t:{:?}", m7);
        let m8 = rnd.gen_range(0..config.t - 1);
        println!("m8 from range z_t:{:?}", m8);
        let m9 = rnd.gen_range(0..config.t - 1);
        println!("m9 from range z_t:{:?}", m9);
        let m10 = rnd.gen_range(0..config.t - 1);
        println!("m10 from range z_t:{:?}", m10);

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
        let (c3 , l3)= encryption::encrypt(
            &BigInt::from(m3),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );
        let (c4,l4) = encryption::encrypt(
            &BigInt::from(m4),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );
        let (c5, _) = encryption::encrypt(
            &BigInt::from(m5),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );
        let (c6, l6) = encryption::encrypt(
            &BigInt::from(m6),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );
        let (c7, l7) = encryption::encrypt(
            &BigInt::from(m7),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );
        let ( c8, l8) = encryption::encrypt(
            &BigInt::from(m8),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );
        let (c9, l9) = encryption::encrypt(
            &BigInt::from(m9),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );
        let (c10, l10) = encryption::encrypt(
            &BigInt::from(m10),
            &setup.bigint_t,
            &setup.params.rhu,
            &setup.p,
            &setup.q,
            &setup.d,
            &setup.N,
            setup.params.bits,
        );

        let modulus_n = BigInt::from(setup.N.clone());
        // let f_test = "c1 * c2 * c3 * c4 * c5  + 7 c6 * c7 +   5 c8 * c9 * c10";
        
        let terms = vec![
            EqPTerm {
                coefficient: BigInt::one(), // No explicit coefficient, so it is 1
                variables: [
                    ("c1".to_string(), 1),
                    ("c2".to_string(), 1),
                    ("c3".to_string(), 1),
                    ("c4".to_string(), 1),
                    ("c5".to_string(), 1),
                ]
                .iter()
                .cloned()
                .collect(),
            }
            ,
            EqPTerm {
                coefficient: BigInt::from(7),
                variables: [("c6".to_string(), 1), ("c7".to_string(), 1)]
                    .iter()
                    .cloned()
                    .collect(),
            }
            ,
            EqPTerm {
                coefficient: BigInt::from(5),
                variables: [
                    ("c8".to_string(), 1),
                    ("c9".to_string(), 1),
                    ("c10".to_string(), 1),
                ]
                .iter()
                .cloned()
                .collect(),
            },
        ];

        let start_eval_equalized = Instant::now();
        let polynomial: EqPPolynomial = EqPPolynomial::new(terms);
        // Define variable values  
        let variables = [ 
            ("c1".to_string(), c1.clone()),
            ("c2".to_string(), c2.clone()),
            ("c3".to_string(), c3.clone()),
            ("c4".to_string(), c4.clone()),
            ("c5".to_string(), c5.clone()),
            ("c6".to_string(), c6.clone()),
            ("c7".to_string(), c7.clone()),
            ("c8".to_string(), c8.clone()),
            ("c9".to_string(), c9.clone()),
            ("c10".to_string(), c10.clone()),
        ]
        .iter()
        .cloned()
        .collect();

        let ( degree_differences ,max_degree) = polynomial.find_degree_differences();
        let array=Fhscheme_Labeled::utility::read_values_from_file("src/data/preprocessed_encrypted_data.txt");
        //println!("max degree:{:?}- degree diffrence:{:?}", max_degree,degree_differences);
        let eval_res = polynomial.evaluate(&variables, &degree_differences, &array ,&modulus_n);
        let decrypted_eval_res = decryption::decrypt_method_one(&eval_res, &setup.p,&setup.q, &d_l_array, &setup.bigint_t, max_degree);
        println!("decrypted_eval_res is :{:?}", decrypted_eval_res);

        let manual_res= &(c1 * c2 * c3 * c4 * c5   + &array[2]* BigInt::from(7) *c6 * c7 +  &array[1] * BigInt::from(5) * c8 * c9 * c10 ) % modulus_n;
        //println!("manual :{:?}",manual_res);
        //println!("ebval :{:?}",eval_res);   
        // let expected_result=(m1*m2*m3 )% &setup.bigint_t;
        // let expected_result=(m1 * m2 *m3 *m4 * m5 )% &setup.bigint_t;
        let expected_result=(m1 *m2*m3*m4*m5 + BigInt::from(7) *m6 * m7 +  BigInt::from(5)* m8 * m9 * m10  )% &setup.bigint_t;
        println!("expected_result is {:?}", expected_result);

        // println!("check test eval 5 vari");
        // assert_eq!(expected_result,decryption::decrypt_method_one(&manual_res, &setup.p, &setup.q, &setup.d, &setup.bigint_t, 5));
        assert_eq!(expected_result, decrypted_eval_res);

        // let on=BigInt::from(1);
        // f(c1,..., c10)= 5(c1*c2 ) + c1*c1*c3 +4 (c5)
        // let start_equalized=Instant::now();

        // let decrypted_custom = decryption::decrypt(&eval(ciphertexts, 10, f_test, &one_enc), &setup.p, &setup.bigint_t);
        // let decrypted_custum_equalized = decryption::decrypt(&eval(ciphertexts, &10, &equalizer(&f_test)), &setup.p, &setup.bigint_t);
        // let eqalized_f=equalizer(&f_test);

        // let end_equalizer=start_equalized.elapsed();
        // println!("equalizer time:{:?}",end_equalizer);

        // let start_eval=Instant::now();
        // eval(ciphertexts, 10, &f_test, &one_enc);
        // let end_eval=start_eval.elapsed();
        // println!(" eval_without_equalized_time: {:?}",end_eval);
        // assert_eq!(eqalized_f, expected_f);
    }
}