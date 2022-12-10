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
 * Separate the input string in the middle and return the item (char) that
 * exists in both parts.
 */
fn find_repeated_item(input: &String) -> char {
    let first_half = &input[0..input.len() / 2];
    let second_half = &input[input.len() / 2..input.len()];
    for c in first_half.chars() {
        if second_half.contains(c) {
            return c;
        }
    }
    return ' ';
}

/**
 * Returns the priority value for a given item (char).
 *
 * The priority value for 'a' to 'z' are 1 to 26 respectively.
 * The priority value for 'A' to 'Z' are 27 to 52 respectively.
 */
fn get_priority(c: char) -> i32 {
    if ( (c as i32) >= ('a' as i32) ) && ( (c as i32) <= ('z' as i32) ) {
        return (c as i32) - ('a' as i32) + 1;
    } else if ( (c as i32) >= ('A' as i32) ) && ( (c as i32) <= ('Z' as i32) ) {
        return (c as i32) - ('A' as i32) + 27;
    } else {
        return 0;
    }
}

/**
 * Calculates the sum of priorities for all the repeated items in each line.
 */
fn part1(input: &Vec<String>) -> i32 {
    let mut total_priorities = 0;
    for line in input {
        let repeated_item = find_repeated_item(line);
        if repeated_item != ' ' {
            total_priorities += get_priority(repeated_item);
        }
    }
    return total_priorities;
}

/**
 * Returns the item (char) that exists in all three input strings.
 */
fn find_common_item(in1: &String, in2: &String, in3: &String) -> char {
    for c in in1.chars() {
        if in2.contains(c) && in3.contains(c) {
            return c;
        }
    }
    return ' ';
}

/**
 * Calculates the sum of priorities for all the common items in every 3 lines.
 */
fn part2(input: &Vec<String>) -> i32 {
    let mut index = 0;
    let mut total_priorities = 0;
    while index < input.len() {
        let in1 = &input[index];
        let in2 = &input[index + 1];
        let in3 = &input[index + 2];
        let common_item = find_common_item(in1, in2, in3);
        if common_item != ' ' {
            total_priorities += get_priority(common_item);
        }
        index += 3;
    }
    return total_priorities;
}

fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}