use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashSet;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}

/**
 * Given the location of two knots, return if they are touching each other.
 *
 * Two knots are touching if they are on the same row, column, or diagonal.
 */
fn is_touching(a: (i32, i32), b: (i32, i32)) -> bool {
    if ((a.0 - b.0).abs() <= 1) && ((a.1 - b.1).abs() <= 1) {
        return true;
    }
    return false;
}

/**
 * Given a direction, return the change in x and y coordinates.
 */
fn get_move(direction: char) -> (i32, i32) {
    match direction {
        'U' => (0, 1),
        'D' => (0, -1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => panic!("Invalid direction"),
    }
}

/**
 * Given the locations of the previous and the current knot,
 * return the updated location of the current knot.
 */
fn do_move(prev: (i32, i32), curr: (i32, i32)) -> (i32, i32) {
    // if the previous and current location are on the same column
    if (prev.0 - curr.0).abs() == 0 {
        if prev.1 > curr.1 {
            return (curr.0, curr.1 + 1);
        } else {
            return (curr.0, curr.1 - 1);
        }

    // if the previous and current location are on the same row
    } else if (prev.1 - curr.1).abs() == 0 {
        if prev.0 > curr.0 {
            return (curr.0 + 1, curr.1);
        } else {
            return (curr.0 - 1, curr.1);
        }

    // if the previous and current location are on the diagonal
    } else {
        if prev.0 > curr.0 {
            if prev.1 > curr.1 {
                return (curr.0 + 1, curr.1 + 1);
            } else {
                return (curr.0 + 1, curr.1 - 1);
            }
        } else {
            if prev.1 > curr.1 {
                return (curr.0 - 1, curr.1 + 1);
            } else {
                return (curr.0 - 1, curr.1 - 1);
            }
        }
    }
}

/**
 * Given the input, return the number of unique locations that the tail knot
 * has visited.
 */
fn part1(input: &Vec<String>) -> i32 {
    // init the location of the head and nine knots to (0, 0)s
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited = HashSet::new();

    // parse each line in the input in the format of "direction distance"
    for line in input {
        let args: Vec<&str> = line.split_whitespace().collect();
        let direction = args[0].chars().nth(0).unwrap();
        let distance = args[1].parse::<i32>().unwrap();

        for _i in 0..distance {
            // update the location of the head
            let (dx, dy) = get_move(direction);
            head = (head.0 + dx, head.1 + dy);

            // update the location of the tail
            if !is_touching(head, tail) {
                tail = do_move(head, tail);
            }

            // add the tail knot to the visited set
            visited.insert(tail);
        }
    }

    return visited.len() as i32;
}

/**
 * Given the input, return the number of unique locations that the tail knot
 * has visited.
 */
fn part2(input: &Vec<String>) -> i32 {
    // init the location of the head and nine knots to (0, 0)s
    let mut knots = [(0, 0); 10];

    let mut visited = HashSet::new();

    // parse each line in the input in the format of "direction distance"
    for line in input {
        let args: Vec<&str> = line.split_whitespace().collect();
        let direction = args[0].chars().nth(0).unwrap();
        let distance = args[1].parse::<i32>().unwrap();

        for _i in 0..distance {
            // update the location of the head
            let (dx, dy) = get_move(direction);
            knots[0] = (knots[0].0 + dx, knots[0].1 + dy);

            // update the location of the nine following knots
            for k in 0..9 {
                if !is_touching(knots[k], knots[k + 1]) {
                    knots[k + 1] = do_move(knots[k], knots[k + 1]);
                }
            }

            // add the tail knot to the visited set
            visited.insert(knots[9]);
        }
    }

    return visited.len() as i32;
}

fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}