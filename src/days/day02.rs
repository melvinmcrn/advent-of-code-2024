use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    println!("Day 02 solution!");

    // Open the input file
    let file = File::open("inputs/day02.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<i32>> = Vec::new();

    // Read each line from the file
    for line in reader.lines() {
        let line = line.expect("Cannot read line");
        let fields: Vec<&str> = line.split_whitespace().collect();

        let mut row: Vec<i32> = Vec::new();

        for num_str in fields {
            let num = num_str.parse::<i32>();

            match num {
                Ok(n) => {
                    row.push(n);
                }
                _ => {
                    println!("Skipping line with invalid number: {}", line);
                    break;
                }
            }
        }

        data.push(row);
    }

    // Part 1: Count safe report
    let mut safe_count: i32 = 0;
    // Part 2: Count safe report with single bad level tolerance
    let mut safe_with_tolerance_count: i32 = 0;

    for report in data {
        if is_safe_report(&report) {
            safe_count += 1;
        }

        if is_safe_report_with_tolerance(&report) {
            safe_with_tolerance_count += 1;
        }
    }

    println!("Safe Reports: {}", safe_count);
    println!("Safe Reports with Tolerance: {}", safe_with_tolerance_count);
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let mut is_checked = false;
    let mut is_safe = true;
    let mut is_increasing = false;

    for i in 0..(report.len() - 1) {
        let num_1 = report[i];
        let num_2 = report[i + 1];

        if !is_checked {
            is_checked = true;
            is_increasing = num_1 < num_2;
        }

        let diff = (num_2 - num_1).abs();

        if (diff > 3)
            || (diff < 1)
            || (is_increasing && num_1 > num_2)
            || (!is_increasing && num_1 < num_2)
        {
            is_safe = false;
            break;
        }
    }

    is_safe
}

fn is_safe_report_with_tolerance(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut cloned_report = report.clone();
        cloned_report.remove(i);

        if is_safe_report(&cloned_report) {
            return true;
        }
    }

    false
}
