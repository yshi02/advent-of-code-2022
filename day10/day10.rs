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
 * Produce a pixel for the print buffer based on the cycle and the register value.
 *
 * reg is a 3-pixel wide sprite
 * if the sprite overlaps with the current cycle, return a '#', otherwise return a '.'
 */
fn produce_pixel(cycle: i32, reg: i32) -> char {
    if ((cycle % 40) - reg).abs() <= 1 {
        return '#';
    } else {
        return '.';
    }
}

/**
 * Calculates the sum of the strength of the signal at each cycle
 *
 * the strength of the signal is the product of the cycle and the register value
 */
fn part1(input: &Vec<String>) -> i32 {
    // init a cycle counter
    let mut cycle = 0;

    // init register X to 1
    let mut reg = 1;

    let mut sum_strength = 0;

    for line in input {
        if line.starts_with("noop") {
            // do nothing
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                sum_strength += reg * cycle;
            }
        } else {
            // parse the operand of the instruction as i32
            let operand = line[5..].parse::<i32>().unwrap();
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                sum_strength += reg * cycle;
            }

            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                sum_strength += reg * cycle;
            }
            reg += operand;
        }
    }

    return sum_strength;
}

/**
 * Executes the program and fills the print buffer at each cycle
 */
fn part2(input: &Vec<String>) -> i32 {
    // init the print buffer
    let mut buffer = Vec::new();

    // init a cycle counter
    let mut cycle = 0;

    // init register X to 1
    let mut reg = 1;

    for line in input {
        buffer.push(produce_pixel(cycle, reg));
        cycle += 1;

        if line.starts_with("noop") {
            // do nothing
        } else {
            // parse the operand of the instruction as i32
            let operand = line[5..].parse::<i32>().unwrap();

            buffer.push(produce_pixel(cycle, reg));
            reg += operand;
            cycle += 1;
        }
    }

    // print the buffer, 40 pixels per line
    for i in 0..buffer.len() {
        if i % 40 == 0 {
            println!("");
        }
        print!("{}", buffer[i]);
    }
    println!("");

    return 0;
}


fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: retval: {}, solution: PBZGRAZA", part2(&lines));
}