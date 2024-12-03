# Advent of Code 2024 - Rust Solutions 🎄

Welcome to my repository for **Advent of Code 2024**! This year, I've decided to take on the challenge using **Rust**, a language I'm exploring for the very first time. Please bear with me as I learn and improve along the way. 😄

## About This Repository
This repository contains my solutions to the puzzles from [Advent of Code 2024](https://adventofcode.com/2024). Each day consists of two parts, and the solutions are written in Rust (`.rs` files). The main goals of this repository are:
- To solve the Advent of Code puzzles.
- To practice and get comfortable with Rust.

## Folder Structure
Here’s an overview of the folder structure for the repository:

```
advent-of-code-2024/
├── inputs/
│   ├── day01.txt         # Input file for Day 1
│   ├── day02.txt         # Input file for Day 2
│   └── ...               # Input files for other days
├── src/
│   ├── days/
│   │   ├── day01.rs      # Solution for Day 1
│   │   ├── day02.rs      # Solution for Day 2
│   │   └── mod.rs        # Module file for day-specific solutions
│   ├── main.rs           # Main entry point for the program
├── .gitignore
├── Cargo.lock
├── Cargo.toml
```

### Details
- **Inputs**: All input files for each day's challenge are stored in the `inputs/` folder, named `dayXX.txt` for convenience.
- **Source Code**: The `src/days/` folder contains individual Rust files (`dayXX.rs`) for each day's solution. The `src/main.rs` file serves as the entry point to run the solutions.
- **Modules**: The `mod.rs` file in the `src/days/` folder is used to manage and organize the day-specific modules.

## Getting Started
To run the solutions, ensure you have Rust installed on your machine. Then, follow these steps:

1. Clone this repository:
```bash
   git clone https://github.com/melvinmcrn/advent-of-code-2024.git
   cd advent-of-code-2024
```

2. Compile and run the solution for a specific day (e.g., Day 1):
```bash
   rustc src/days/day01.rs -o day01
   ./day01
```

3. Alternatively, modify the `src/main.rs` to call the desired day's solution, and then run:
```bash
   cargo run
```

4. The program will automatically read the corresponding input file from the `inputs/` directory.

## Why Rust?

Rust is known for its performance, safety, and modern tooling. As a beginner in Rust, I saw Advent of Code as a perfect opportunity to get hands-on experience with:
- File handling.
- Ownership, borrowing, and lifetimes.
- Algorithms and data structures.
- Error handling and debugging in Rust.

## Disclaimer

Since I’m learning Rust as I solve these puzzles, some of the code might not follow best practices or idiomatic Rust conventions. Constructive feedback and suggestions are welcome! 😊

## Contributing

Feel free to submit pull requests or open issues for improvements, optimizations, or alternative approaches.

---
Thank you for visiting, and happy coding!

ps. this markdown is generated by ChatGPT 🤖