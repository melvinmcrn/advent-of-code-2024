use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    println!("Day 08 solution!");

    // Open the input file
    let file = File::open("inputs/day08.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut data: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            line.expect("Cannot read line")
                .trim()
                .chars()
                .collect::<Vec<char>>()
        })
        .collect();

    for i in 0..lines.len() {
        let line: &Vec<char> = &lines[i];

        for j in 0..line.len() {
            let c: char = line[j];

            if c == '.' {
                continue;
            }

            data.entry(c)
                .or_insert_with(|| Vec::new())
                .push((i as i32, j as i32));
        }
    }

    let max_length: (i32, i32) = (lines.len() as i32, lines[0].len() as i32);

    // results is keep as `x-y` format
    // while x is column and y is row
    let mut results_part_1: HashSet<String> = HashSet::new();
    let mut results_part_2: HashSet<String> = HashSet::new();

    for (_, positions) in data.iter() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let pos1: (i32, i32) = positions[i];
                let pos2: (i32, i32) = positions[j];

                let diff_x: i32 = pos1.0 - pos2.0;
                let diff_y: i32 = pos1.1 - pos2.1;

                let mut result_1: (i32, i32) = (pos1.0 + diff_x, pos1.1 + diff_y);
                let mut result_2: (i32, i32) = (pos2.0 - diff_x, pos2.1 - diff_y);

                // part 1
                if !is_out_of_bounds(max_length, result_1) {
                    let formatted_result_1: String = format!("{}-{}", result_1.0, result_1.1);
                    results_part_1.insert(formatted_result_1.clone());
                    results_part_2.insert(formatted_result_1.clone());
                }

                if !is_out_of_bounds(max_length, result_2) {
                    let formatted_result_2: String = format!("{}-{}", result_2.0, result_2.1);
                    results_part_1.insert(formatted_result_2.clone());
                    results_part_2.insert(formatted_result_2.clone());
                }

                // part 2
                results_part_2.insert(format!("{}-{}", pos1.0, pos1.1));
                results_part_2.insert(format!("{}-{}", pos2.0, pos2.1));

                loop {
                    result_1 = (result_1.0 + diff_x, result_1.1 + diff_y);

                    if is_out_of_bounds(max_length, result_1) {
                        break;
                    }

                    let formatted_result_1: String = format!("{}-{}", result_1.0, result_1.1);
                    results_part_2.insert(formatted_result_1.clone());
                }

                loop {
                    result_2 = (result_2.0 - diff_x, result_2.1 - diff_y);

                    if is_out_of_bounds(max_length, result_2) {
                        break;
                    }

                    let formatted_result_2: String = format!("{}-{}", result_2.0, result_2.1);
                    results_part_2.insert(formatted_result_2.clone());
                }
            }
        }
    }

    println!("Part 1: Total results: {}", results_part_1.len());
    println!("Part 2: Total results: {}", results_part_2.len());
}

pub fn is_out_of_bounds(max_length: (i32, i32), pos: (i32, i32)) -> bool {
    pos.0 < 0 || pos.1 < 0 || pos.0 >= max_length.0 || pos.1 >= max_length.1
}
