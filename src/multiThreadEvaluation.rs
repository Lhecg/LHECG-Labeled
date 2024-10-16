use num_bigint::BigInt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use num_traits::One;

#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: BigInt,
    pub variables: HashMap<String, i32>,  // Stores variables and their powers
}

#[derive(Debug)]
pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    pub fn new(terms: Vec<Term>) -> Self {
        Polynomial { terms }
    }

    pub fn multievaluate(&self, variables: &HashMap<String, BigInt>) -> BigInt {
        let variables = Arc::new(Mutex::new(variables.clone()));
        let mut handles = vec![];
        let mut result = BigInt::from(0);

        for term in &self.terms {
            let term = term.clone();
            let variables = Arc::clone(&variables);

            let handle = thread::spawn(move || {
                let mut term_value = term.coefficient.clone();
                let vars = variables.lock().unwrap();

                for (var, power) in &term.variables {
                    if let Some(value) = vars.get(var) {
                        let mut variable_power = BigInt::one();
                        for _ in 0..*power {
                            variable_power *= value;
                        }
                        term_value *= variable_power;
                    } else {
                        term_value = BigInt::from(0);
                        break;
                    }
                }
                
                term_value
            });

            handles.push(handle);
        }

        for handle in handles {
            result += handle.join().unwrap();
        }

        result
    }
}
