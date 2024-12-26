use std::collections::HashSet;

pub fn main() {
    println!("Day 10 solution!");

    // Read the input file
    let data_str: &str = include_str!("../../inputs/day10.txt");
    let data_grid: Vec<Vec<u8>> = data_str
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect::<Vec<u8>>()
        })
        .collect();

    // Part 1
    let mut unique_trail_count: usize = 0;

    // Part 2
    let mut full_trail_count: usize = 0;

    for row in 0..data_grid.len() {
        for col in 0..data_grid[row].len() {
            let num: u8 = data_grid[row][col];

            if num != 0 {
                continue;
            }

            // This is the trail head
            let mut trail_ends: Vec<(usize, usize)> = Vec::new();

            find_trail_path((row, col), &data_grid, &mut trail_ends);

            full_trail_count += trail_ends.len();

            let unique_trail_ends: HashSet<(usize, usize)> = trail_ends.into_iter().collect();

            unique_trail_count += unique_trail_ends.len();
        }
    }

    // Part 1
    println!("Unique Trail Count: {}", unique_trail_count);

    // Part 2
    println!("Full Trail Count: {}", full_trail_count);
}

fn find_trail_path(
    position: (usize, usize),
    data_grid: &Vec<Vec<u8>>,
    trail_ends: &mut Vec<(usize, usize)>,
) {
    let current_height = data_grid[position.0][position.1];

    if current_height == 9 {
        trail_ends.push(position);
        return;
    }

    // N
    if position.0 > 0
        && check_trail_step(current_height, ((position.0 - 1), position.1), &data_grid)
    {
        find_trail_path((position.0 - 1, position.1), &data_grid, trail_ends);
    }

    // S
    if position.0 < data_grid.len() - 1
        && check_trail_step(current_height, ((position.0 + 1), position.1), &data_grid)
    {
        find_trail_path((position.0 + 1, position.1), &data_grid, trail_ends);
    }

    // W
    if position.1 > 0
        && check_trail_step(current_height, (position.0, (position.1 - 1)), &data_grid)
    {
        find_trail_path((position.0, position.1 - 1), &data_grid, trail_ends);
    }

    // E
    if position.1 < data_grid[position.0].len() - 1
        && check_trail_step(current_height, (position.0, (position.1 + 1)), &data_grid)
    {
        find_trail_path((position.0, position.1 + 1), &data_grid, trail_ends);
    }
}

fn check_trail_step(
    current_height: u8,
    next_position: (usize, usize),
    data_grid: &Vec<Vec<u8>>,
) -> bool {
    let next_height = data_grid[next_position.0 as usize][next_position.1 as usize];

    if current_height + 1 == next_height {
        return true;
    }

    false
}
