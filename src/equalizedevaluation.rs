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

    pub fn find_degree_differences(&self) -> Vec<i32> {
        let mut max_degree = 0;
        let mut term_degrees = Vec::with_capacity(self.terms.len());

        for term in &self.terms {
            let mut term_degree = 0;
            for power in term.variables.values() {
                term_degree += power;
            }
            if term_degree > max_degree {
                max_degree = term_degree;
            }
            term_degrees.push(term_degree);
        }

        term_degrees.into_iter()
            .map(|degree| max_degree - degree)
            .collect()
    }

    pub fn evaluate(&self, variable_values: &HashMap<String, BigInt>, degree_differences: &Vec<i32>) -> BigInt {
        let mut result = BigInt::from(0);

        for (i, term) in self.terms.iter().enumerate() {
            let mut term_result = term.coefficient.clone();

            for (var, &power) in &term.variables {
                if let Some(value) = variable_values.get(var) {
                    let var_term = value.pow(power as u32);
                    term_result *= var_term;
                }
            }

            let multiplier = BigInt::from(2).pow(degree_differences[i] as u32);
            term_result *= multiplier;

            result += term_result;
        }

        result
    }
}
