use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::time::Instant;
use std::env;

fn get_accessible_rolls(grid: &[Vec<u32>], row_idx: usize, col_idx: usize) -> u32 {
    if grid[row_idx][col_idx] == 0 {
        return 0;
    }
    
    let row_len: usize = grid[row_idx].len();
    let col_len: usize = grid.len();

    // iterate row offset from -1 to 1
    // iterate col offset from -1 to 1
    // if the neighbor is a 1, add it to the sum
    // return the sum
    let mut sum = 0;
    for row_offset in -1..=1 as i32 {
        for col_offset in -1..=1 as i32 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }
            let neighbor_row: i32 = row_idx as i32 + row_offset;
            let neighbor_col: i32 = col_idx as i32 + col_offset;
            if neighbor_row < 0 || neighbor_row >= col_len as i32 || neighbor_col < 0 || neighbor_col >= row_len as i32 {
                continue;
            }
            if grid[neighbor_row as usize][neighbor_col as usize] == 1 {
                sum += 1;
            }
        }
    }

    let accessible = sum < 4;
    let accessible_score: u32 = accessible as u32;

    accessible_score
}

fn main() -> io::Result<()> {

    let input = std::fs::read_to_string("input.txt").expect("Unable to read input file");

    // convert grid into ones and zeros
    let grid: Vec<Vec<u32>> = input.lines().map(|row| row.chars().map(|c| if c == '@' { 1 } else { 0 }).collect()).collect();
    println!("grid: {:?}", grid);

    let puzzle_1: Vec<Vec<u32>> = (0..grid.len())
        .map(|row_idx| {
            (0..grid[0].len())
                .map(|col_idx| get_accessible_rolls(&grid, row_idx, col_idx))
                .collect()
        })
        .collect();

    // println!("puzzle_1: {:?}", puzzle_1);

    println!("part 1 answer: {}", puzzle_1.iter().map(|v| v.iter().sum::<u32>()).sum::<u32>());

    // for part 2, we will do this iteratively. 
    // for each itertion, get the same accessible grid. if its sum is > 0, then subtract it elementwise from the original and continue
    // if sum is 0, that's the end condition.
    
    let mut puzzle_2_grid: Vec<Vec<u32>> = grid.clone();
    let mut puzzle_2: Vec<Vec<u32>> = puzzle_1.clone();
    while puzzle_2.iter().map(|v| v.iter().sum::<u32>()).sum::<u32>() > 0 {
        // subtract puzzle 2 from puzzle 2 grid elementwise
        for i in 0..puzzle_2_grid.len() {
            for j in 0..puzzle_2_grid[i].len() {
                puzzle_2_grid[i][j] -= puzzle_2[i][j];
            }
        }
        // get the new accessible grid
        puzzle_2 = (0..puzzle_2_grid.len())
            .map(|row_idx| {
                (0..puzzle_2_grid[0].len())
                    .map(|col_idx| get_accessible_rolls(&puzzle_2_grid, row_idx, col_idx))
                    .collect()
            })
            .collect();
    }
    println!("part 2 answer: {}", grid.iter().map(|v| v.iter().sum::<u32>()).sum::<u32>() - puzzle_2_grid.iter().map(|v| v.iter().sum::<u32>()).sum::<u32>());

    Ok(())
}