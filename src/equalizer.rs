use num_bigint::BigInt;



pub fn equalizer(expression: &str) -> String {
    let max_degree = expression.split('+')
        .map(|term| term.matches('*').count() + 1)  // Degree is the count of '*' + 1
        .max()
        .unwrap();

    let mut equalized_f = String::new();
    for term in expression.split('+') {
        let trimmed_term = term.trim();
        let degree = trimmed_term.matches('*').count() + 1;

        let k_factor = if degree < max_degree {
            max_degree - degree
        } else {
            0
        };

        let k_multiplication = " k * ".repeat(k_factor);
            let equalized_term = if k_factor > 0 {
                let pos = trimmed_term.find('(').unwrap_or(0) + 1; // +1 to insert after '('
            if trimmed_term.starts_with('(') {
                format!("{}{}{}", &trimmed_term[..pos], k_multiplication, &trimmed_term[pos..].trim_start())
            } else {
                format!("({} {})", "k * ".repeat(k_factor), trimmed_term)
            }
        } else {
            trimmed_term.to_string()
        };
        //         // format!("( {}{} )", trimmed_term.trim_start_matches("("),k_multiplication)
    
        //         let pos = term.find('(').unwrap_or(0) + 1; // +1 to insert after '('
            
        //         format!("{} k_multiplication {}", &trimmed_term[..pos], &trimmed_term[pos..].trim_start())
        // } else {
        //     trimmed_term.to_string()
        // };
       
    

        equalized_f.push_str(&equalized_term);
        // equalized_f.push_str(&format!("( {} )", equalized_term));

        equalized_f.push_str(" + ");
    }

    equalized_f.trim_end_matches(" + ").to_string()
}



// 
