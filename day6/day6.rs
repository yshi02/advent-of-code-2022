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
 * Check if the 4 characters in a rolling window contains at least two of the same character.
 * If so, the first character gets processed.
 *
 * Returns the number of characters to be processed.
 */
fn part1(input: &Vec<String>) -> i32 {
    let mut num_chars_to_be_processed = 0;

    for i in 0..input[0].chars().count() - 4 {
        let c0 = input[0].chars().nth(i).unwrap();
        let c1 = input[0].chars().nth(i + 1).unwrap();
        let c2 = input[0].chars().nth(i + 2).unwrap();
        let c3 = input[0].chars().nth(i + 3).unwrap();
        if c0 == c1 || c0 == c2 || c0 == c3 || c1 == c2 || c1 == c3 || c2 == c3 {
            num_chars_to_be_processed += 1;
        } else {
            break;
        }
    }

    return num_chars_to_be_processed + 4;
}

/**
 * Check if the 14 characters in a rolling window contains at least two of the same character.
 * If so, the first character gets processed.
 *
 * Returns the number of characters to be processed.
 */
fn part2(input: &Vec<String>) -> i32 {
    let mut num_chars_to_be_processed = 0;

    // if 14 consecutive characters contain at least two of the same character,
    // then the first character gets processed
    for i in 0..input[0].chars().count() - 14 {
        let mut tmp = Vec::new();
        for j in 0..14 {
            let c = input[0].chars().nth(i + j).unwrap();
            if tmp.iter().any(|&x| x == c) {
                num_chars_to_be_processed += 1;
                break;
            } else {
                tmp.push(c);
                if tmp.len() == 14 {
                    return num_chars_to_be_processed + 14;
                }
            }
        }
    }

    return -1;
}



fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}