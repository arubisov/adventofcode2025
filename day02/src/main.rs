use std::fs;
use std::io; // For Result<T, E> in main function
// use std::iter;

fn main() -> io::Result<()> {
    // this doesn't work - needs a session cookie.
    // let url = "https://adventofcode.com/2025/day/2/input";
    // let body = reqwest::blocking::get(url)?.text()?;

    let puzzle: u8 = 2;

    let file_path = "input.txt";
    let file_content = fs::read_to_string(file_path)?;
    println!("File content:\n{}", file_content);
    
    let parts = file_content.trim().split(','); 

    let mut answer: u64 = 0;
    let mut all_invalid = Vec::new();

    for part in parts {
        let (lower, upper) = part.split_once('-').expect("failed to split over dash");
        let lower_num = lower.parse::<u64>().unwrap();
        let upper_num = upper.parse::<u64>().unwrap();

        // puzzle 1
        if puzzle == 1 {
            // answer: 13919717792
            // for each part pair (lower, upper), contstruct all the invalid numbers that are 
            // greater than lower and less than upper do so my initializing an empty vector and 
            // then pushing the invalid numbers to it using the next_invalid_number function
            // do this in a while loop that continues until the number is greater than upper
            let mut number = lower_num;
            
            loop {
                let next_invalid = next_invalid_number(number);    
                if next_invalid > upper_num {
                    break;
                }
                answer += next_invalid;
                number = next_invalid + 1;
            }
        }

        // puzzle 2
        // generate all the invalid numbers between lower and upper incrementally by increasing the
        // base number and the number of repetitions; break once exceeding the upper. 
        if puzzle == 2 {
            let invalid_numbers = generate_invalid_numbers(lower_num, upper_num);
            all_invalid.extend(invalid_numbers.clone());
            println!("lower: {}, upper: {}, invalid_numbers: {:?}", lower, upper, invalid_numbers);
            answer += invalid_numbers.iter().sum::<u64>();
        }
    }

    // print the sum of all the invalid numbers
    println!("answer: {}", answer);

    // check if there were dups in the all_invalid?
    all_invalid.sort();
    all_invalid.dedup();
    println!("all_invalid: {:?}", all_invalid.iter().sum::<u64>());

    Ok(())
}

// define a function that takes an integer and returns the next invalid number that is greater than or equal to the integer
fn next_invalid_number(number: u64) -> u64 {

    let num_str: &str = &number.to_string();

    if num_str.len() % 2 == 0 && num_str[0..num_str.len()/2] == num_str[num_str.len()/2..] {
        // println!("number: {}, response: {}", number, number);
        return num_str.parse::<u64>().unwrap();
    }

    let base: String;

    if num_str.len() % 2 == 1 {
        let num_zeros: u32 = ((num_str.len()-1)/2).try_into().unwrap();
        base = i32::pow(10, num_zeros).to_string();
    } else if num_str.len() % 2 == 0 && num_str[0..num_str.len()/2] < num_str[num_str.len()/2..] {
        let inc = num_str[0..num_str.len()/2].parse::<u64>().unwrap() + 1;
        base = inc.to_string();
    } else if num_str.len() % 2 == 0 && num_str[0..num_str.len()/2] > num_str[num_str.len()/2..] {
        base = num_str[0..num_str.len()/2].to_string();
    } else {
        base = "0".to_string();
    }

    let response = (base.clone() + &base).parse::<u64>().unwrap();
    // println!("number: {}, response: {}", number, response);

    return response;
}

// generate all the invalid numbers between lower and upper incrementally by increasing the
// base number and the number of repetitions; break once exceeding the upper. 
fn generate_invalid_numbers(lower: u64, upper: u64) -> Vec<u64> {
    let mut invalid = Vec::new();
    
    // start with pattern "1" and work upward
    let mut pattern = 1u64;
    
    loop {
        let pattern_str = pattern.to_string();
        
        // increment the number of repeats, starting at 1
        let mut repetitions = 2;
        loop {
            let repeated = pattern_str.repeat(repetitions);
            
            // idiomatic Rust way to try this and proceed only if it works
            if let Ok(num) = repeated.parse::<u64>() {
                if num > upper {
                    // break out of this loop, increase the base pattern
                    break;
                }
                if num >= lower {
                    invalid.push(num);
                }
                // increment here - may have been below lower and not met previous if condition
                repetitions += 1; 
            } else {
                // overflow for u64
                break;
            }
        }
        
        // terminate if a double base pattern already exceeds upper, otherwise increment the base pattern
        if let Ok(num) = (pattern_str.clone() + &pattern_str).parse::<u64>() {
            if num > upper {
                break;
            }
        } else {
            break;
        }
        
        pattern += 1;
    }
    
    invalid.sort();
    invalid.dedup();
    // this is the idiomatic Rust way to return a variable at the end of a function, omit the semicolon
    invalid
}