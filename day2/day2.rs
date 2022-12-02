use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}

/**
 * Calculates the total score for the Rock Paper Scissors game according to the
 * strategy guide.
 *
 * Win = 6 pts, Draw = 3 pts, Lose = 0pts.
 * Rock (A/X) = 1 pts, Paper (B/Y) = 2pts, Scissors (C/Z) = 3pts.
 */
fn part1(input: &Vec<String>) -> i32 {
    let mut total_score = 0;
    for line in input {
        let mut score = 0;
        let opponent_choice = line.chars().nth(0).unwrap();
        let my_choice = line.chars().nth(2).unwrap();

        score += match (opponent_choice, my_choice) {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // win
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // tie
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0, // lose
            _ => 0,
        };

        score += match my_choice {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };

        total_score += score;
    }
    return total_score;
}

/**
 * Calculates the total score for the Rock Paper Scissors game according to the
 * strategy guide.
 *
 * Win (Z) = 6 pts, Draw (Y) = 3 pts, Lose (X) = 0pts.
 * Rock (A) = 1 pts, Paper (B) = 2pts, Scissors (C) = 3pts.
 */
fn part2(input: &Vec<String>) -> i32 {
    let mut total_score = 0;
    for line in input {
        let mut score = 0;
        let opponent_choice = line.chars().nth(0).unwrap();
        let outcome = line.chars().nth(2).unwrap();

        score += match (opponent_choice, outcome) {
            ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1, // we choose Rock
            ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2, // we choose Paper
            ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3, // we choose Scissors
            _ => 0,
        };

        score += match outcome {
            'Z' => 6,
            'Y' => 3,
            'X' => 0,
            _ => 0,
        };

        total_score += score;
    }
    return total_score;
}


fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}