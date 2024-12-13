use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    println!("Day 06 solution!");

    // Open the input file
    let file = File::open("inputs/day06.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut map_data: Vec<Vec<bool>> = Vec::new();
    let mut guard_position: (i32, i32) = (0, 0);
    let mut guard_direction: (i32, i32) = (0, 0);

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

        let mut row: Vec<bool> = Vec::new();

        for j in 0..line.len() {
            let c = line[j];

            if c == '#' {
                row.push(true);
            } else if c == '.' {
                row.push(false);
            } else {
                row.push(false);
                guard_position = (i as i32, j as i32);

                // switch case for the direction, elgiible options are: > < ^ v
                guard_direction = match c {
                    '>' => (0, 1),
                    '<' => (0, -1),
                    '^' => (-1, 0),
                    'v' => (1, 0),
                    _ => panic!("Invalid guard direction"),
                };
            }
        }

        map_data.push(row);
    }

    // part 1
    let walked_positions = simulate_walk_positions(&map_data, guard_position, guard_direction)
        .expect("Error while counting walked positions");
    println!("Number of walked positions: {}", walked_positions.len());

    // part 2
    let count_eligible_obstacle = simulate_extra_obstacle(
        &map_data,
        guard_position,
        guard_direction,
        &walked_positions,
    );
    println!("Number of eligible obstacles: {}", count_eligible_obstacle);
}

fn simulate_extra_obstacle(
    map_data: &Vec<Vec<bool>>,
    guard_position: (i32, i32),
    guard_direction: (i32, i32),
    walked_positions: &HashSet<(i32, i32)>,
) -> usize {
    println!(
        "Simulating extra obstacle with {} possible positions...",
        walked_positions.len()
    );

    let mut count_eligible_obstacle: usize = 0;

    // loop each position in walked_positions
    for position in walked_positions.iter() {
        // if the position is the guard position, skip
        if position == &guard_position {
            continue;
        }

        let mut new_map_data = map_data.clone();
        new_map_data[position.0 as usize][position.1 as usize] = true;

        let new_walked_positions =
            simulate_walk_positions(&new_map_data, guard_position, guard_direction);

        // check if simulated walk positions panick
        match new_walked_positions {
            Ok(_) => continue,
            Err(_) => count_eligible_obstacle += 1,
        };
    }

    count_eligible_obstacle
}

fn simulate_walk_positions(
    map_data: &Vec<Vec<bool>>,
    guard_position: (i32, i32),
    guard_direction: (i32, i32),
) -> Result<HashSet<(i32, i32)>, String> {
    let mut walked_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut walked_positions_with_direction: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    let mut current_position = guard_position;
    let mut current_direction = guard_direction;

    loop {
        let positions_with_direction = (
            current_position.0,
            current_position.1,
            current_direction.0,
            current_direction.1,
        );

        if walked_positions_with_direction.contains(&positions_with_direction) {
            return Err("Duplicate position found".to_string());
        }

        walked_positions.insert(current_position);
        walked_positions_with_direction.insert(positions_with_direction);

        let new_position = (
            current_position.0 + current_direction.0,
            current_position.1 + current_direction.1,
        );

        // out of range
        if new_position.0 < 0
            || new_position.1 < 0
            || new_position.0 >= map_data.len() as i32
            || new_position.1 >= map_data[0].len() as i32
        {
            break;
        }

        // found obstacle -> change direction
        if map_data[new_position.0 as usize][new_position.1 as usize] == true {
            current_direction = match current_direction {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("Invalid guard direction"),
            };

            continue;
        }

        current_position = new_position;
    }

    Ok(walked_positions)
}
