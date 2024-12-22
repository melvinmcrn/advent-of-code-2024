use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    println!("Day 09 Part 1 solution!");

    // Open the input file
    let file = File::open("inputs/day09.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            line.expect("Cannot read line")
                .trim()
                .chars()
                .collect::<Vec<char>>()
        })
        .collect();

    let mut data: Vec<u8> = Vec::new();
    let mut spaces: Vec<u8> = Vec::new();

    let mut is_space = false;
    for c in lines.get(0).unwrap() {
        let number: u8 = c.to_digit(10).expect("Cannot parse number") as u8;
        if is_space {
            spaces.push(number);
        } else {
            data.push(number);
        }

        is_space = !is_space;
    }

    let mut is_space: bool = false;

    let mut data_index: usize = 0;
    let mut space_index: usize = 0;
    let mut data_space_index: usize = data.len() - 1;

    let mut index: usize = 0;

    let mut check_sum: u64 = 0;

    loop {
        if data_index == data_space_index && data[data_index] == 0 {
            break;
        }

        if !is_space {
            if data[data_index] == 0 {
                is_space = true;
                data_index += 1;
                continue;
            } else {
                check_sum += (index * data_index) as u64;
                data[data_index] = data[data_index] - 1;
            }
        } else {
            if data[data_space_index] == 0 {
                data_space_index -= 1;
            }

            if spaces[space_index] == 0 {
                is_space = false;
                space_index += 1;
                continue;
            } else {
                check_sum += (index * data_space_index) as u64;
                spaces[space_index] = spaces[space_index] - 1;
                data[data_space_index] = data[data_space_index] - 1;
            }
        }

        index += 1;
    }

    println!("Check Sum: {}", check_sum);
}
