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
 * Calculates running totals for the given input and returns the largest value.
 *
 * The calorie values for different Efls are separated by a newline.
 */
fn part1(input: &Vec<String>) -> i32 {
    let mut max_total = 0;
    let mut current_total = 0;
    for line in input {
        match line.parse::<i32>() {
            Ok(n) => current_total += n,
            Err(_) => current_total = 0,
        }
        if current_total > max_total {
            max_total = current_total;
        }
    }
    max_total
}

/**
 * Calculates the sum of the top three running sums for the given input.
 *
 * The calorie values for different Efls are separated by a newline.
 */
fn part2(input: &Vec<String>) -> i32 {
    let mut totals = Vec::new();
    let mut current_total = 0;
    for line in input {
        match line.parse::<i32>() {
            Ok(n) =>  current_total += n,
            Err(_) => {
                totals.push(current_total);
                current_total = 0;
            },
        }
    }
    totals.sort();
    totals[totals.len() - 1] + totals[totals.len() - 2] + totals[totals.len() - 3]
}

fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}