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
 * Parse the input into a two dimensional vector of integers and the start and end positions.
 *
 * The a-z characters are converted to 0-25, and the start and end positions are
 * converted to 0 and 25 respectively.
 */
fn parse_input(input: &Vec<String>) -> (Vec< Vec<i32> >, (i32, i32), (i32, i32)) {
    let mut heightmap = Vec::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    for i in 0..input.len() {
        let mut row = Vec::new();
        for j in 0..input[i].len() {
            let c = input[i].chars().nth(j).unwrap();
            if c == 'S' {
                row.push(0);
                start = (i as i32, j as i32);
            } else if c == 'E' {
                row.push(25);
                end = (i as i32, j as i32);
            } else {
                row.push(c as i32 - 'a' as i32);
            }
        }
        heightmap.push(row);
    }
    return (heightmap, start, end);
}

/**
 * Returns the shortest path from the start to the end.
 */
fn part1(input: &Vec<String>) -> i32 {
    let (heightmap, start, end) = parse_input(input);

    // start breadth first search from the start
    let mut steps: Vec<Vec<i32>> = vec![vec![i32::MAX - 1; heightmap[0].len()]; heightmap.len()];
    let mut queue = Vec::new();

    // initialize the start position
    queue.push(start);
    steps[start.0 as usize][start.1 as usize] = 0;

    while queue.len() > 0 {

        let current = queue.remove(0);
        let current_steps = steps[current.0 as usize][current.1 as usize];

        // check all 4 directions
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for direction in directions {
            let next = (current.0 + direction.0, current.1 + direction.1);

            // check if the next heuristics is a valid position
            if next.0 < 0 || next.0 >= heightmap.len() as i32 || next.1 < 0 || next.1 >= heightmap[0].len() as i32 {
                continue;
            }

            // check if we can move to the next position
            let current_height = heightmap[current.0 as usize][current.1 as usize];
            let next_height = heightmap[next.0 as usize][next.1 as usize];
            if next_height > current_height + 1 {
                continue;
            }

            if steps[next.0 as usize][next.1 as usize] > current_steps + 1 {
                steps[next.0 as usize][next.1 as usize] = current_steps + 1;
                queue.push(next);
            }

        }
    }

    let end_steps = steps[end.0 as usize][end.1 as usize];

    return end_steps;
}

/**
 * Returns the shortest path from the base level (0 or 'a') to the end.
 */
fn part2(input: &Vec<String>) -> i32 {
    let (heightmap, _start, end) = parse_input(input);

    // start breadth first search from the end
    let mut steps: Vec<Vec<i32>> = vec![vec![i32::MAX - 1; heightmap[0].len()]; heightmap.len()];
    let mut queue = Vec::new();

    // initialize the start position
    queue.push(end);
    steps[end.0 as usize][end.1 as usize] = 0;

    // the search terminates when we reach a height 0, or 'a' this time
    let mut valid_steps = Vec::new();

    while queue.len() > 0 {

        let current = queue.remove(0);
        let current_steps = steps[current.0 as usize][current.1 as usize];

        if heightmap[current.0 as usize][current.1 as usize] == 0 {
            valid_steps.push(current_steps);
            continue;
        }

        // check all 4 directions
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for direction in directions {
            let next = (current.0 + direction.0, current.1 + direction.1);

            // check if the next heuristics is a valid position
            if next.0 < 0 || next.0 >= heightmap.len() as i32 || next.1 < 0 || next.1 >= heightmap[0].len() as i32 {
                continue;
            }

            // check if we can move to the next position
            let current_height = heightmap[current.0 as usize][current.1 as usize];
            let next_height = heightmap[next.0 as usize][next.1 as usize];
            if next_height < current_height - 1 {
                continue;
            }

            if steps[next.0 as usize][next.1 as usize] > current_steps + 1 {
                steps[next.0 as usize][next.1 as usize] = current_steps + 1;
                queue.push(next);
            }

        }
    }

    // find the minimum steps that are valid
    let mut end_steps = i32::MAX;
    for step in valid_steps {
        if step < end_steps {
            end_steps = step;
        }
    }

    return end_steps;
}

fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}