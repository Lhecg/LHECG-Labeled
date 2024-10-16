use num_bigint::{BigInt, BigUint};
use std::error::Error;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use std::{fs::File, io::BufRead};

pub fn save_parameters_to_file(
    filename: &str,
    p: &BigUint,
    q: &BigUint,
    d:&BigInt,
    N: &BigUint,
) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", p)?;
    writeln!(writer, "{}", q)?;
    writeln!(writer, "{}", d)?;

    writeln!(writer, "{}", N)?;
    writer.flush()?;
    Ok(())
}
pub fn load_parameters_from_file(filename: &str) -> io::Result<(BigUint, BigUint, BigInt, BigUint)> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut p_str = String::new();
    let mut q_str = String::new();
    let mut d_str = String::new();

    let mut n_str = String::new();

    reader.read_line(&mut p_str)?;
    reader.read_line(&mut q_str)?;
    reader.read_line(&mut d_str)?;

    reader.read_line(&mut n_str)?;

    let p = BigUint::from_str(p_str.trim()).unwrap();
    let q = BigUint::from_str(q_str.trim()).unwrap();
    let d = BigUint::from_str(d_str.trim()).unwrap();

    let N = BigUint::from_str(n_str.trim()).unwrap();

    Ok((p, q,d.into(), N))
}

// pub fn read_preprocessed_data(filename: &str) -> [BigInt; 10] {
//     let mut array: [BigInt; 10] = Default::default(); //

//     // Fill array with BigInt::from(0)
//     for item in array.iter_mut() {
//         *item = BigInt::from(1);
//     }

//     let file = File::open(filename).expect("Unable to open file");
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
        // println!("Processing line: {:?}", line); // Debugging output

// // // // //         let line = line.expect("Unable to read line");
// // // // //         let parts: Vec<&str> = line.split(':').collect();
// // // // //         // println!("parts:{:?}", parts);
// // // // //         if parts.len() == 2 {
// // // // //             // println!("part:{:?}", parts[0].trim().parse::<usize>()
// // // // //     // );
// // // // //             if let Ok(index) = parts[0].trim().parse::<usize>() {
// // // // //                 println!("index:{:?}",index);
// // // // //                 if index > 0 && index <= 10 {
// // // // //                     let value = BigInt::parse_bytes(parts[1].trim().as_bytes(), 10)
// // // // //                         .expect("Unable to parse value");

// // // // //                     array[index - 1] = value;
// // // // //                 }
// // // // //             }
// // // // //         }
// // // // //     }

// // // // //     array
// // // // // }



// // // // use std::fs::File;
// // // // use std::io::{BufRead, BufReader};
// // // // use num_bigint::BigInt;
// // // // use num_traits::FromPrimitive;

// // // // pub fn read_preprocessed_data(filename: &str) -> [BigInt; 10] {
// // // //     let mut array: [BigInt; 10] = Default::default();

// // // //     // Initialize array with BigInt::from(0)
// // // //     for item in array.iter_mut() {
// // // //         *item = BigInt::from(0);
// // // //     }

// // // //     let file = File::open(filename).expect("Unable to open file");
// // // //     let reader = BufReader::new(file);

// // // //     for line in reader.lines() {
// // // //         let line = line.expect("Unable to read line");
// // // //         let parts: Vec<&str> = line.split(':').collect();

// // // //         if parts.len() == 2 {
// // // //             let index_str = parts[0].trim();
// // // //             let value_str = parts[1].trim();

// // // //             // Debugging output
// // // //             println!("Index part: '{}'", index_str);
// // // //             println!("Value part: '{}'", value_str);

// // // //             // Handle index parsing as usize
// // // //             match index_str.parse::<usize>() {
// // // //                 Ok(index) => {
// // // //                     if index > 0 && index <= 10 {
// // // //                         // Handle value parsing as BigInt
// // // //                         match BigInt::parse_bytes(value_str.as_bytes(), 10) {
// // // //                             Ok(value) => {
// // // //                                 array[index - 1] = value;
// // // //                             }
// // // //                             Err(_) => {
// // // //                                 eprintln!("Failed to parse value: '{}'", value_str);
// // // //                             }
// // // //                         }
// // // //                     } else {
// // // //                         eprintln!("Index out of range: {}", index);
// // // //                     }
// // // //                 }
// // // //                 Err(_) => {
// // // //                     eprintln!("Failed to parse index: '{}'", index_str);
// // // //                 }
// // // //             }
// // // //         } else {
// // // //             eprintln!("Line format is incorrect: '{}'", line);
// // // //         }
// // // //     }

// // // //     array
// // // // }

// // // use std::fs::File;
// // // use std::io::{BufRead, BufReader};
// // // use num_bigint::BigInt;
// // // use num_traits::FromPrimitive;

// // // pub fn read_values_from_file(filename: &str) -> [BigInt; 10] {
// // //     let mut array: [BigInt; 10] = Default::default();
    
// // //     // Initialize array with BigInt::from(0)
// // //     for item in array.iter_mut() {
// // //         *item = BigInt::from(0);
// // //     }

// // //     // Open the file and create a buffered reader
// // //     let file = File::open(filename).expect("Unable to open file");
// // //     let reader = BufReader::new(file);

// // //     // Iterate over each line in the file
// // //     for line in reader.lines() {
// // //         let line = line.expect("Unable to read line");
// // //         let parts: Vec<&str> = line.split(':').collect();

// // //         // Process lines with exactly two parts
// // //         if parts.len() == 2 {
// // //             let index_str = parts[0].trim();
// // //             let value_str = parts[1].trim();

// // //             // Parse the index as usize
// // //             if let Ok(index) = index_str.strip_prefix("k").unwrap_or("").parse::<usize>() {
// // //                 // Check if the index is within range
// // //                 if index > 0 && index <= 10 {
// // //                     // Parse the value as BigInt
// // //                     // if let Ok(value) =
// // //                    let value=  BigInt::parse_bytes(value_str.as_bytes(), 10) {
// // //                         array[index - 1] = value;
// // //                     // } else {
// // //                     //     eprintln!("Failed to parse value: '{}'", value_str);
// // //                     // }
// // //                 } else {
// // //                     eprintln!("Index out of range: {}", index);
// // //                 }
// // //             } else {
// // //                 eprintln!("Failed to parse index: '{}'", index_str);
// // //             }
// // //         } else {
// // //             eprintln!("Line format is incorrect: '{}'", line);
// // //         }
// // //     }

// // //     array
// // // }


// // use std::fs::File;
// // use std::io::{BufRead, BufReader};
// // use num_bigint::BigInt;
// // use num_traits::FromPrimitive;

// // pub fn read_values_from_file(filename: &str) -> [BigInt; 10] {
// //     let mut array: [BigInt; 10] = Default::default();

// //     // Open the file and create a buffered reader
// //     let file = File::open(filename).expect("Unable to open file");
// //     let reader = BufReader::new(file);

// //     // Iterate over each line in the file
// //     for line in reader.lines() {
// //         let line = line.expect("Unable to read line");
// //         let parts: Vec<&str> = line.split(':').collect();

// //         // Process lines with exactly two parts
// //         if parts.len() == 2 {
// //             let index_str = parts[0].trim();
// //             let value_str = parts[1].trim();

// //             // Parse the index as usize
// //             if let Ok(index) = index_str.strip_prefix("k").unwrap_or("").parse::<usize>() {
// //                 // Check if the index is within range
// //                 if index > 0 && index <= 10 {
// //                     // Parse the value as BigInt
// //                     let value = match BigInt::parse_bytes(value_str.as_bytes(), 10) {
// //                         Ok(val) => val,
// //                         Err(_) => {
// //                             eprintln!("Failed to parse value: '{}'", value_str);
// //                             continue; // Skip to next line
// //                         }
// //                     };

// //                     array[index - 1] = value;
// //                 } else {
// //                     eprintln!("Index out of range: {}", index);
// //                 }
// //             } else {
// //                 eprintln!("Failed to parse index: '{}'", index_str);
// //             }
// //         } else {
// //             eprintln!("Line format is incorrect: '{}'", line);
// //         }
// //     }

// //     array
// // }
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use num_bigint::BigInt;
// use num_traits::FromPrimitive;

// pub fn read_values_from_file(filename: &str) -> [BigInt; 10] {
//     let mut array: [BigInt; 10] = Default::default();

//     // Open the file and create a buffered reader
//     let file = File::open(filename).expect("Unable to open file");
//     let reader = BufReader::new(file);

//     let mut index = 0;

//     // Iterate over each line in the file
//     for line in reader.lines() {
//         let line = line.expect("Unable to read line");
//         let value_str = line.trim();

//         // Parse the value as BigInt
//         match BigInt::parse_bytes(value_str.as_bytes(), 10) {
//             Ok(value) => {
//                 if index < 10 {
//                     array[index] = value;
//                     index += 1;
//                 } else {
//                     eprintln!("Array is full. More values than expected.");
//                     break;
//                 }
//             },
//             Err(_) => eprintln!("Failed to parse value: '{}'", value_str),
//         }
//     }

//     array
// }


pub fn read_values_from_file(filename: &str) -> [BigInt; 10] {
    let mut array: [BigInt; 10] = Default::default();
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut index = 0;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let value_str = line.trim();

        // Parse the value as BigInt
        let value = match BigInt::parse_bytes(value_str.as_bytes(), 10) {
            Some(val) => val,
            None => {
                eprintln!("Failed to parse value: '{}'", value_str);
                continue; 
            }
        };

        if index < 10 {
            array[index] = value;
            index += 1;
        } else {
            eprintln!("Array is full. More values than expected.");
            break;
        }
    }

    array
}
