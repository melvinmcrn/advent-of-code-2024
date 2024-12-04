use std::fs;

pub fn main() {
    println!("Day 04 solution!");

    // read file as string
    let input = fs::read_to_string("inputs/day04.txt").expect("Cannot read file");

    let data_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Part 1: Find XMAS
    let mut word_count = 0;
    // Part 2: Find Cross-MAS
    let mut cross_count = 0;

    for row in 0..data_grid.len() {
        for col in 0..data_grid[row].len() {
            word_count += check_word(&data_grid, row, col);
            cross_count += check_cross(&data_grid, row, col);
        }
    }

    println!("Word count: {}", word_count);
    println!("Cross count: {}", cross_count);
}

fn check_cross(data_grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let word_chars: Vec<char> = "MAS".chars().collect();

    // the center must be 'A'
    if data_grid[row][col] != 'A' {
        return 0;
    }

    // check if out of bounds
    if row == 0 || row == data_grid.len() - 1 || col == 0 || col == data_grid[0].len() - 1 {
        return 0;
    }

    // check NE <> SW
    if !(check_direction(data_grid, &word_chars, row - 1, col + 1, 1, -1) > 0
        || check_direction(data_grid, &word_chars, row + 1, col - 1, -1, 1) > 0)
    {
        return 0;
    };
    // check NW <> SE
    if !(check_direction(data_grid, &word_chars, row - 1, col - 1, 1, 1) > 0
        || check_direction(data_grid, &word_chars, row + 1, col + 1, -1, -1) > 0)
    {
        return 0;
    };

    1
}

fn check_word(data_grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let word_chars: Vec<char> = "XMAS".chars().collect();
    let mut word_count = 0;

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

    word_count
}

fn check_direction(
    data_grid: &[Vec<char>],
    word_chars: &[char],
    start_row: usize,
    start_col: usize,
    row_step: isize,
    col_step: isize,
) -> usize {
    let rows = data_grid.len() as isize;
    let cols = data_grid[0].len() as isize;
    let word_len = word_chars.len();

    for i in 0..word_len {
        let r: isize = (start_row as isize) + (i as isize) * row_step;
        let c: isize = (start_col as isize) + (i as isize) * col_step;

        // check if out of bounds
        if r < 0 || r >= rows || c < 0 || c >= cols {
            return 0;
        }

        // check if character matches word
        if data_grid[r as usize][c as usize] != word_chars[i as usize] {
            return 0;
        }
    }

    1
}
