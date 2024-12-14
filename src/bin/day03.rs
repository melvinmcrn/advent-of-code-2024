use regex::Regex;
use std::fs;

pub fn main() {
    println!("Day 03 solution!");

    // read file as string
    let data = fs::read_to_string("inputs/day03.txt").expect("Cannot read file");

    let pattern = r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))";
    let re = Regex::new(pattern).unwrap();

    let regex_result = re.find_iter(&data);

    // Part 1: sum all matches
    let mut sum_result_1 = 0;

    // Part 2: sum all matches when enabled
    let mut sum_result_2 = 0;
    let mut is_enable = true;

    for result in regex_result {
        let result_str = result.as_str();

        if result_str.contains("do()") {
            is_enable = true;
            continue;
        } else if result_str.contains("don't()") {
            is_enable = false;
            continue;
        }

        let truncated_str = &result_str.replace("mul(", "").replace(")", "");
        let splited_str: Vec<&str> = truncated_str.split(",").collect();
        let splitted_num = splited_str
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mul_num = splitted_num[0] * splitted_num[1];

        if is_enable {
            sum_result_2 += mul_num;
        }

        sum_result_1 += mul_num;
    }

    println!("Sum of all results 1: {}", sum_result_1);
    println!("Sum of all results 2: {}", sum_result_2);
}
