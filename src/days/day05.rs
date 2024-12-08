use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    println!("Day 05 solution!");

    // Open the input file
    let file = File::open("inputs/day05.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut page_ordering_rules: HashSet<String> = HashSet::new();
    let mut updates: Vec<String> = Vec::new();

    let mut found_break_line = false;

    // page ordering rules
    for line_result in reader.lines() {
        let line = line_result.expect("Cannot read line");
        let trimmed_line = line.trim().to_string();

        if !found_break_line {
            // Check if the line is empty
            if trimmed_line.is_empty() {
                found_break_line = true;
                continue;
            }

            page_ordering_rules.insert(trimmed_line);
        } else {
            updates.push(trimmed_line);
        }
    }

    // part 1
    let mut sum_middle: i32 = 0;

    // part 2
    let mut sum_middle_invalid: i32 = 0;

    for update in updates {
        let mut fields: Vec<&str> = update.split(',').collect();

        let mut is_line_valid = true;

        for i in 0..(fields.len() - 1) {
            for j in i..fields.len() {
                let field_i = fields[i];
                let field_j = fields[j];

                // check for ordering rules that will make this update incorrect
                let formatted_rule = format!("{}|{}", field_j, field_i);

                if page_ordering_rules.contains(&formatted_rule) {
                    is_line_valid = false;
                    fields.swap(i, j);
                }
            }
        }

        let parsed_middle: i32 = fields[fields.len() / 2]
            .parse::<i32>()
            .expect("Cannot parse integer");

        if is_line_valid {
            sum_middle += parsed_middle;
        } else {
            sum_middle_invalid += parsed_middle;
        }
    }

    println!("Sum of middle values: {}", sum_middle);
    println!("Sum of middle values (invalid): {}", sum_middle_invalid);
}
