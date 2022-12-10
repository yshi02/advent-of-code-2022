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
 * Get all graphically represented crates from the input file.
 *
 * crates are in the format
 *     [D]
 * [N] [C]
 * [Z] [M] [P]
 *  1   2   3
 */
fn get_crates(input: &Vec<String>) -> Vec<Vec<char>> {
    // get the number of crates and the bottom line number of the crates
    let mut num_crates = 0;
    let mut bottom_line = 0;
    for line in input {
        if line.starts_with(" 1") {
            num_crates = line.chars().nth(line.len() - 2).unwrap().to_digit(10).unwrap();
            break;
        }
        bottom_line += 1;
    }

    // create a vector of crates
    let mut crates = Vec::new();
    for _ in 0..num_crates {
        crates.push(Vec::new());
    }

    // add the crates to the vector
    for i in 0..num_crates {
        for line in input[..bottom_line].iter().rev() {
            if line.chars().nth(1 + (4 * i as usize)).unwrap() != ' ' {
                crates[i as usize].push(line.chars().nth(1 + (4 * i as usize)).unwrap());
            } else {
                break;
            }
        }
    }

    return crates;
}

/**
 * Performs the moves according to the instructions and returns and items on the top of the crates.
 *
 * Moves are performed one at a time.
 */
fn part1(input: &Vec<String>) -> String {
    let mut crates = get_crates(input);

    let mut result_string = String::new();

    for line in input {
        if line.starts_with("move") {
            let src: u32 = line[line.find("from ").unwrap() + 5 .. line.find(" to ").unwrap()].parse().unwrap();
            let dst: u32 = line[line.find(" to ").unwrap() + 4 .. line.len()].parse().unwrap();
            let num: u32 = line[line.find("move ").unwrap() + 5 .. line.find(" from ").unwrap()].parse().unwrap();

            // move the crates
            for _ in 0..num {
                let item = crates[src as usize - 1].pop().unwrap();
                crates[dst as usize - 1].push(item);
            }
        }
    }

    for c in crates {
        result_string.push_str(c.last().unwrap().to_string().as_str());
    }

    return result_string;
}

/**
 * Performs the moves according to the instructions and returns and items on the top of the crates.
 *
 * Moves are performed in batches.
 */
fn part2(input: &Vec<String>) -> String {
    let mut crates = get_crates(input);

    let mut result_string = String::new();

    for line in input {
        if line.starts_with("move") {
            let src: u32 = line[line.find("from ").unwrap() + 5 .. line.find(" to ").unwrap()].parse().unwrap();
            let dst: u32 = line[line.find(" to ").unwrap() + 4 .. line.len()].parse().unwrap();
            let num: u32 = line[line.find("move ").unwrap() + 5 .. line.find(" from ").unwrap()].parse().unwrap();

            // move the crates
            let mut temp = Vec::new();
            for _ in 0..num {
                temp.push(crates[src as usize - 1].pop().unwrap());
            }
            while !temp.is_empty() {
                crates[dst as usize - 1].push(temp.pop().unwrap());
            }
        }
    }

    for c in crates {
        result_string.push_str(c.last().unwrap().to_string().as_str());
    }

    return result_string;
}


fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}