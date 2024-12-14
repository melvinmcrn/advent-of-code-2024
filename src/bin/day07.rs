use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    println!("Day 07 solution!");

    // Open the input file
    let file = File::open("inputs/day07.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut results: Vec<u64> = Vec::new();
    let mut eqations: Vec<Vec<u64>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Cannot read line");
        let fields: Vec<&str> = line.trim().split(": ").collect();

        let result: u64 = fields[0].parse().expect("Cannot parse result");
        let equation: Vec<u64> = fields[1]
            .split_whitespace()
            .map(|x| x.parse().expect("Cannot parse equation"))
            .collect();

        results.push(result);
        eqations.push(equation);
    }

    // Part 1: Find the sum of valid results
    let mut sum_valid_results: u64 = 0;
    for i in 0..results.len() {
        if check_is_valid_equation(0, &eqations[i], results[i]) {
            sum_valid_results += results[i];
        }
    }

    println!("Sum of valid results: {}", sum_valid_results);
}

pub fn check_is_valid_equation(previous_result: u64, equation: &Vec<u64>, result: u64) -> bool {
    check_sum(previous_result, &equation, result)
        || check_multiply(previous_result, &equation, result)
        || check_concat(previous_result, &equation, result) // remove this line for part 1
}

pub fn check_sum(previous_result: u64, remaining: &Vec<u64>, result: u64) -> bool {
    let new_result: u64 = previous_result + remaining[0];

    // this is the last one
    if remaining.len() == 1 {
        if new_result == result {
            return true;
        } else {
            return false;
        }
    } else {
        // continue the recursion
        let new_remaining: Vec<u64> = remaining[1..].to_vec();

        return check_is_valid_equation(new_result, &new_remaining, result);
    }
}

pub fn check_multiply(previous_result: u64, remaining: &Vec<u64>, result: u64) -> bool {
    let new_result: u64 = previous_result * remaining[0];

    // this is the last one
    if remaining.len() == 1 {
        if new_result == result {
            return true;
        } else {
            return false;
        }
    } else {
        // continue the recursion
        let new_remaining: Vec<u64> = remaining[1..].to_vec();

        return check_is_valid_equation(new_result, &new_remaining, result);
    }
}

pub fn check_concat(previous_result: u64, remaining: &Vec<u64>, result: u64) -> bool {
    let new_result: u64 = format!("{}{}", previous_result, remaining[0])
        .parse::<u64>()
        .expect("Cannot parse concatenated number");

    // this is the last one
    if remaining.len() == 1 {
        if new_result == result {
            return true;
        } else {
            return false;
        }
    } else {
        // continue the recursion
        let new_remaining: Vec<u64> = remaining[1..].to_vec();

        return check_is_valid_equation(new_result, &new_remaining, result);
    }
}
