use num_bigint::BigInt;
use num_traits::One;
use std::collections::HashMap;

// Define a structure for a term
#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: BigInt,
    pub variables: HashMap<String, i32>,  
}

#[derive(Debug)]
pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    pub fn new(terms: Vec<Term>) -> Self {
        Polynomial { terms }
    }

    pub fn find_degree_differences(&self) ->  (Vec<i32> , usize){
        let mut max_degree = 0;
        let mut term_degrees = Vec::with_capacity(self.terms.len());

        for term in &self.terms {
            let mut term_degree = 0;
            for power in term.variables.values() {
                term_degree += power;
                //label*power=degre
            }
            if term_degree > max_degree {
                max_degree = term_degree;
            }
            term_degrees.push(term_degree);
        }

       ( term_degrees.into_iter()
            .map(|degree| max_degree - degree)
            .collect() , max_degree.try_into().unwrap())
    }

    pub fn evaluate(&self, variable_values: &HashMap<String, BigInt>, degree_differences: &Vec<i32>, array: &[BigInt; 10], module:&BigInt) -> BigInt {
        let mut result = BigInt::from(0);
        // let mut term_result=

        for (i, term) in self.terms.iter().enumerate() {
            let mut term_result = term.coefficient.clone();
// println!("term result is {:?}",term_result);
println!("i is {} term is :{:?}",i, term);
            for (var, &power) in &term.variables {
                if let Some(value) = variable_values.get(var) {
                    let var_term = value.pow(power as u32);
                    term_result = term_result * var_term;

                    // println!("term_result:{}",term_result);
                    // println!("power:{}",power);
                }
            }



            // let multiplier = BigInt::from(2).pow(degree_differences[i] as u32);
            // term_result *= multiplier;
            let diff = degree_differences[i];
            if diff > 0 && (diff as usize) < array.len() {
                let multiplier = &array[(diff -1) as usize];
                // println!("multiplier {:?}",multiplier,);
                term_result = term_result * multiplier ;
            }

            result += term_result;
        }


      result % module
    }
}
