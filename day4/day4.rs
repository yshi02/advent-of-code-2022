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
 * Counts the number of cases where one range /contains/ the other.
 *
 * The ranges are specified as a pair of numbers, and the ranges are inclusive.
 * Each line in the input contains two ranges, separated by a space ','
 */
fn part1(input: &Vec<String>) -> i32 {
    let mut contain_cnt = 0;
    for line in input {
        let mut ranges = line.split(",");
        let range1 = ranges.next().unwrap().split("-").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let range2 = ranges.next().unwrap().split("-").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if (range1[0] <= range2[0] && range2[1] <= range1[1])       // range1 contains range2
            || (range2[0] <= range1[0] && range1[1] <= range2[1]) { // range2 contains range1
            contain_cnt += 1;
        }
    }
    return contain_cnt;
}

/**
 * Counts the number of cases where one range /overlaps/ the other.
 *
 * The ranges are specified as a pair of numbers, and the ranges are inclusive.
 * Each line in the input contains two ranges, separated by a space ','
 */
fn part2(input: &Vec<String>) -> i32 {
    let mut overlap_cnt = 0;
    for line in input {
        let mut ranges = line.split(",");
        let range1 = ranges.next().unwrap().split("-").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let range2 = ranges.next().unwrap().split("-").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if range1[0] <= range2[1] && range1[1] >= range2[0] {   // one range overlaps the other
            overlap_cnt += 1;
        }
    }
    return overlap_cnt;
}


fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}