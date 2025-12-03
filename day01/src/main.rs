use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::time::Instant;

fn main() -> io::Result<()> {
    let start = Instant::now();
    let file_path = "src/day01_large.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut pos: i32 = 50;
    let mut pos_rem_prev: i32 = pos.rem_euclid(100);
    let mut answer: i32 = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        // println!("{}", line);
        
        let (direction, distance) = line.split_at(1);
        let distance = distance.parse::<i32>().unwrap();
        let pos_prev: i32 = pos;
        if direction == "L" {
            pos = pos - distance;
        } else {
            pos = pos + distance;
        }

        // puzzle 1
        pos = pos % 100;
        if pos == 0 {
            answer += 1;
        }

        // puzzle 2
        answer += distance.div_euclid(100);
        let pos_rem_curr = pos.rem_euclid(100);

        // detect wraparound
        if pos_prev != 0 && pos != 0 {
            if (direction == "R" && pos_rem_curr < pos_rem_prev) ||
                (direction == "L" && pos_rem_curr > pos_rem_prev) {
                answer += 1;
            }
        }
        pos_rem_prev = pos_rem_curr;

        // println!("line: {}, pos: {}, answer: {}", line, pos, answer);
    }
    let duration = start.elapsed();
    println!("Runtime: {:.2?}", duration);
    println!("Answer: {}", answer);
    Ok(())
}