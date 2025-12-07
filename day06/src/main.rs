fn puzzle_1() -> u64 {

    let input = std::fs::read_to_string("input.txt").expect("Unable to read input file");
    let grid = input.lines().map(|line| line.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    // println!("grid: {:?}", grid);

    let mut answer = 0;
    for col in 0..grid[0].len() {
        if grid.last().unwrap()[col] == "*" {
            // multiply
            answer += grid.iter().take(grid.len() - 1).map(|row| row[col].parse::<u64>().unwrap()).product::<u64>();
        } else {
            // then we sum
            answer += grid.iter().take(grid.len() - 1).map(|row| row[col].parse::<u64>().unwrap()).sum::<u64>();
        }
    }
    answer
}

fn puzzle_2() -> u64 {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read input file");
    
    let operations: Vec<char> = input.lines().last().unwrap().chars().collect();
    println!("operations: {:?}", operations);

    let grid: Vec<Vec<char>> = input.lines().take(input.lines().count() - 1).map(|line| line.chars().collect()).collect();

    let mut answer = 0;

    let mut opp = ' ';

    let nums: Vec<u64> = (0..grid[0].len())
        .fold(vec![], |mut nums, col| {
            if operations[col] != ' ' {
                opp = operations[col]
            }
            let s = grid.iter()
                .map(|row| row[col])
                .collect::<String>();
            let num = s.trim();
            if num.is_empty() {
                if opp == '*' {
                    // multiply
                    println!("multiplying: {:?}", nums);
                    answer += nums.iter().product::<u64>();
                    nums = Vec::new();
                } else {
                    // then we sum
                    println!("summing: {:?}", nums);
                    answer += nums.iter().sum::<u64>();
                    nums = Vec::new();
                }
            } else {
                nums.push(num.parse::<u64>().unwrap());
            }
            nums
        });

    if opp == '*' {
        // multiply
        println!("multiplying: {:?}", nums);
        answer += nums.iter().product::<u64>();
    } else {
        // then we sum
        println!("summing: {:?}", nums);
        answer += nums.iter().sum::<u64>();
    }

    println!("grid: {:?}", grid);
    println!("nums: {:?}", nums);

    answer
}

fn main() {
    let answer_1 = puzzle_1();
    let answer_2 = puzzle_2();

    println!("answer_1: {}", answer_1);

    println!("answer_2: {:?}", answer_2);
}   
