use std::collections::HashMap;

pub fn main() {
    println!("Day 11 solution!");

    // Read the input file
    let data_str: &str = include_str!("../../inputs/day11.txt");
    let stones: Vec<String> = data_str.split_whitespace().map(|s| s.to_string()).collect();

    println!(
        "Stones after 25 count: {}",
        stones
            .iter()
            .map(|stone| blink(stone.clone(), 25))
            .sum::<usize>()
    );

    println!(
        "Stones after 75 count: {}",
        stones
            .iter()
            .map(|stone| blink(stone.clone(), 75))
            .sum::<usize>()
    );
}

fn blink(stone_0: String, iterations: usize) -> usize {
    // id => count
    let mut stones: HashMap<String, usize> = HashMap::new();
    stones.insert(stone_0, 1);

    for _ in 0..iterations {
        let mut new_stones: HashMap<String, usize> = HashMap::new();
        for (stone, count) in stones {
            let blink_result: Vec<String> = blink_stone(&stone);

            for new_stone in blink_result {
                *new_stones.entry(new_stone).or_insert(0) += count;
            }
        }
        stones = new_stones;
    }

    stones.values().sum()
}

fn blink_stone(stone: &String) -> Vec<String> {
    if stone == "0" {
        return ["1".to_string()].to_vec();
    }

    if stone.len() % 2 == 0 {
        let (left, right) = stone.split_at(stone.len() / 2);
        return [
            left.parse::<u64>().unwrap().to_string(),
            right.parse::<u64>().unwrap().to_string(),
        ]
        .to_vec();
    }

    return [(stone.parse::<u64>().unwrap() * 2024).to_string()].to_vec();
}
