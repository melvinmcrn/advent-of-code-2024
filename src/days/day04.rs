use std::fs;

pub fn main() {
    println!("Day 04 solution!");

    // read file as string
    let input = fs::read_to_string("inputs/day04.txt").expect("Cannot read file");

    let data_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let word_chars: Vec<char> = "XMAS".chars().collect();

    let mut word_count = 0;

    for row in 0..data_grid.len() {
        for col in 0..data_grid[row].len() {
            /*
              horizontal
            */
            // horizontal E
            word_count += check_direction(&data_grid, &word_chars, row, col, 0, 1);
            // horizontal W
            word_count += check_direction(&data_grid, &word_chars, row, col, 0, -1);

            /*
              vertical
            */
            // vertical N
            word_count += check_direction(&data_grid, &word_chars, row, col, 1, 0);
            // vertical S
            word_count += check_direction(&data_grid, &word_chars, row, col, -1, 0);

            /*
              diagonal
            */
            // diagonal NE
            word_count += check_direction(&data_grid, &word_chars, row, col, 1, 1);
            // diagonal NW
            word_count += check_direction(&data_grid, &word_chars, row, col, 1, -1);
            // diagonal SE
            word_count += check_direction(&data_grid, &word_chars, row, col, -1, 1);
            // diagonal SW
            word_count += check_direction(&data_grid, &word_chars, row, col, -1, -1);
        }
    }

    println!("Word count: {}", word_count);
}

fn check_direction(
    grid: &[Vec<char>],
    word: &[char],
    start_row: usize,
    start_col: usize,
    row_step: isize,
    col_step: isize,
) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let word_len = word.len();

    for i in 0..word_len {
        let r: isize = (start_row as isize) + (i as isize) * row_step;
        let c: isize = (start_col as isize) + (i as isize) * col_step;

        // check if out of bounds
        if r < 0 || r >= rows || c < 0 || c >= cols {
            return 0;
        }

        // check if character matches word
        if grid[r as usize][c as usize] != word[i as usize] {
            return 0;
        }
    }

    1
}
