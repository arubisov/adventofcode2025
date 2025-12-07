fn puzzle_1(mut grid: Vec<Vec<char>>) -> u64 {
    let mut splits: u64 = 0;

    for row in 0..grid.len()-1 {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'S' {
                grid[row+1][col] = '|'
            }

            if grid[row][col] == '|' {
                if grid[row+1][col] == '^' {
                    splits += 1;
                    grid[row+1][col-1] = '|';
                    grid[row+1][col+1] = '|';
                } else {
                    grid[row+1][col] = '|';
                }
            }
        }
    }

    splits
}

fn puzzle_2(mut grid: Vec<Vec<char>>) -> u64 {
    
    let cols = grid[0].len();

    // dynamic programming
    let mut counts = vec![0u64; cols];
    let mut next_counts = vec![0u64; cols];

    

    for col in 0..cols {
        if grid[0][col] == 'S' {
            counts[col] = 1;
        }
    }

    for row in 1..grid.len() - 1 {
        next_counts.fill(0);

        for col in 0..cols {
            if counts[col] > 0 {
                if grid[row+1][col] == '^' {
                    next_counts[col-1 as usize] += counts[col];
                    next_counts[col+1 as usize] += counts[col];
                } else {
                    next_counts[col] += counts[col];
                }
            } 
        }

        std::mem::swap(&mut counts, &mut next_counts);
    }
    
    counts.iter().sum()
}



fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // println!("grid: {:?}", grid);

    let answer1 = puzzle_1(grid.clone());
    let answer2 = puzzle_2(grid);

    println!("answer_1: {:?}", answer1);
    println!("answer_2: {:?}", answer2);
}