use std::collections::HashSet;

// assumes the ranges are already sorted by lower
fn merge_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    ranges.iter().copied().fold(vec![], |mut acc, (lower, upper)| {
        if let Some(last) = acc.last_mut() {
            if lower <= last.1 + 1 {
                // don't push a new range, just update the last range
                last.1 = last.1.max(upper);
                return acc;
            }
        }
        acc.push((lower, upper));
        acc
    })
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read input file");

    // extract two separate blocks from the input, which are separated by an empty line
    let blocks = input.split("\n\n").collect::<Vec<&str>>();
    
    // iterate over first block lines, split each line over dash into two numbers, parse as u64, and assign first to "lower" and second to "upper"
    // construct a vector of tuples of (lower, upper)

    let mut ranges = blocks[0].lines().map(|line| {
        let (lower, upper) = line.split_once('-').expect("failed to split over dash");
        let lower: u64 = lower.parse::<u64>().unwrap();
        let upper: u64 = upper.parse::<u64>().unwrap();
        (lower, upper)
    }).collect::<Vec<(u64, u64)>>();
    println!("ranges: {:?}", ranges);

    // iterate over second block lines, map them all to a collect into a vector of u64
    let ids = blocks[1].lines().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    println!("ids: {:?}", ids);

    let answer = ids.iter().filter(|id| {
        ranges.iter().any(|(lower, upper)| id >= &lower && id <= &upper)
    }).count();
    println!("part 1 answer: {}", answer);

    // for part 2, we'll need to sort and merge the ranges first.
    ranges.sort_by_key(|(lower, _)| *lower);
    let merged_ranges = merge_ranges(&ranges);
    
    let answer_2 = merged_ranges.iter().fold(0, |acc, (lower, upper)| acc + (upper - lower + 1));
    println!("part 2 answer: {}", answer_2);

}


