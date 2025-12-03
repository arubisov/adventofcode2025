use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::time::Instant;
use std::env;


fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    
    let puzzle: u8 = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    
    println!("Running puzzle {}", puzzle);

    let start = Instant::now();
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut answer: u64 = 0;

    for line_result in reader.lines() {
        // first digit is max from 0 to len-1
        // second digit is max from first digit to end

        let line = line_result?;

        if puzzle == 1 {
            // get max and argmax from 0 to len-1
            let first = line[0..line.len()-1].chars().max().unwrap();
            let argmax = line[0..line.len()-1].chars().position(|c| c == first).unwrap();
            // get max from after first digit to end
            let second = line[argmax+1..].chars().max().unwrap();

            // concat the two digits and parse as an int
            let number = format!("{}{}", first, second).parse::<u64>().unwrap();
            println!("line: {}, number: {}", line, number);
            answer += number;
        } else if puzzle == 2 {
            // now we need to construct a 12-digit number, instead of a 2-digit number
            let mut digits = Vec::new();
            let first = line[0..line.len()-12].chars().max().unwrap();
            let mut argmax = line[0..line.len()-12].chars().position(|c| c == first).unwrap();
            digits.push(first);
            for i in (0..11).rev() {
                let slice_start = argmax + 1;
                let slice_end = line.len() - i;
                let next_digit = line[slice_start..slice_end].chars().max().unwrap();
                let rel_pos = line[slice_start..slice_end].chars().position(|c| c == next_digit).unwrap();
                argmax = slice_start + rel_pos;
                digits.push(next_digit);
            }
            let number = digits.iter().collect::<String>().parse::<u64>().unwrap();
            println!("line: {}, number: {}", line, number);
            answer += number;
        }
    }
    let duration = start.elapsed();
    println!("Runtime: {:.2?}", duration);
    println!("Answer: {}", answer);
    Ok(())
}