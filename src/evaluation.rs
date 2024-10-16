
use num_bigint::BigInt;
use std::collections::HashMap;
use num_traits::One;

#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: BigInt,
    pub variables: HashMap<String, i32>,  // Stores variable and its power
}

#[derive(Debug)]
pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    pub fn new(terms: Vec<Term>) -> Self {
        Polynomial { terms }
    }

    pub fn evaluate(&self, variables: &HashMap<String, BigInt>) -> BigInt {  // Ehsan: why it does not get coefficients as input?
        let mut result = BigInt::from(0);

        for term in &self.terms {
            let mut term_value = term.coefficient.clone();

            for (var, power) in &term.variables {
                if let Some(value) = variables.get(var) {
                    let mut variable_power = BigInt::one();
                    for _ in 0..*power {
                        variable_power *= value;
                    }
                    term_value *= variable_power;
                } else {
                    // If the variable is not provided, assume its value is 0
                    term_value = BigInt::from(0);
                    break;
                }
            }

            result += term_value;
        }

        result
    }
}


// use crate::encryption;

// pub fn eval(ciphertexts: Vec<BigInt>, level: usize, poly: &str, one_enc:&BigInt) -> BigInt {

    

//     // parallel computation on terms
//     let mut variables: HashMap<String, BigInt> = HashMap::new();
//     for (i, ciphertext) in ciphertexts.iter().enumerate() {
//         variables.insert(format!("c{}", i + 1), ciphertext.clone());
//         // println!("i is: {:?}", i);
//     }
//     // let k=BigInt::from(1);



//     // let poly_k: String = poly.replace("k", &one_enc.to_string());

//     evaluate_expression(&poly, &variables)
// }

// fn evaluate_expression(expression: &str, variables: &HashMap<String, BigInt>) -> BigInt {
//     let tokens = tokenize(expression);
//     // println!("tokens: {:?}", tokens);

//     let postfix = infix_to_postfix(&tokens);
//     // println!("postfix: {:?}", postfix);

//     evaluate_postfix(&postfix, variables)
// }

// fn tokenize(expression: &str) -> Vec<String> {
//     let mut tokens = Vec::new();
//     let mut current_token = String::new();

//     for c in expression.chars() {
//         match c {
//             ' ' => {
//                 if !current_token.is_empty() {
//                     tokens.push(current_token.clone());
//                     current_token.clear();
//                 }
//             }
//             '(' | ')' | '*' | '+' => {
//                 if !current_token.is_empty() {
//                     tokens.push(current_token.clone());
//                     current_token.clear();
//                 }
//                 tokens.push(c.to_string());
//             }
//             _ => current_token.push(c),
//         }
//     }

//     if !current_token.is_empty() {
//         tokens.push(current_token);
//     }

//     tokens
// }

// fn infix_to_postfix(tokens: &[String]) -> Vec<String> {
//     let mut output = Vec::new();
//     let mut operators = Vec::new();

//     let precedence = |op: &str| match op {
//         "*" => 2,
//         "+" => 1,
//         "(" => 0,
//         ")" => 0,
//         _ => -1,
//     };

//     for token in tokens {
//         match token.as_str() {
//             "(" => operators.push(token.clone()),
//             ")" => {
//                 while let Some(op) = operators.pop() {
//                     if op == "(" {
//                         break;
//                     }
//                     output.push(op);
//                 }
//             }
//             "*" | "+" => {
//                 while let Some(op) = operators.last() {
//                     if precedence(op) >= precedence(token) {
//                         output.push(operators.pop().unwrap());
//                     } else {
//                         break;
//                     }
//                 }
//                 operators.push(token.clone());
//             }
//             _ => output.push(token.clone()),
//         }
//     }

//     while let Some(op) = operators.pop() {
//         output.push(op);
//     }

//     output
// }

// fn evaluate_postfix(tokens: &[String], variables: &HashMap<String, BigInt>) -> BigInt {
//     let mut stack = Vec::new();

//     for token in tokens {
//         if let Ok(value) = token.parse::<BigInt>() {
//             stack.push(value);
//         } else if let Some(value) = variables.get(token) {
//             stack.push(value.clone());
//         } else if token == "*" || token == "+" {
//             if stack.len() < 2 {
//                 panic!("Not enough operands for operation: {}", token);
//             }
//             let b = stack.pop().expect("Invalid expression");
//             let a = stack.pop().expect("Invalid expression");

//             let result = match token.as_str() {
//                 "*" => a * b,
//                 "+" => a + b,
//                 _ => unreachable!(),
//             };

//             stack.push(result);
//         } else {
//             panic!("Unknown token: {}", token);
//         }
//     }

//     if stack.len() != 1 {
//         panic!("Invalid expression. Stack size is not 1 after evaluation.");
//     }
//     stack.pop().expect("Invalid expression")
// }