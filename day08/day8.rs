use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut tree_map = Vec::new();
    for line in input {
        let mut row = Vec::new();
        let mut i = 0;
        while i < line.len() {
            row.push(line.chars().nth(i).unwrap().to_digit(10).unwrap() as i32);
            i += 1;
        }
        tree_map.push(row);
    }
    return tree_map;
}

/**
 *
 */
fn part1(input: &Vec<String>) -> i32 {
    let mut covered_num = 0;
    let tree_map = parse_input(input);

    for i in 1..tree_map.len()-1 {
        for j in 1..tree_map[i].len()-1 {
            // check if tree_map[i][j] is covered in all directions
            let mut covered = 0;
            // check for left
            for li in 0..i {
                if tree_map[li][j] >= tree_map[i][j] {
                    covered += 1;
                    break;
                }
            }
            // check for right
            for ri in i+1..tree_map.len() {
                if tree_map[ri][j] >= tree_map[i][j] {
                    covered += 1;
                    break;
                }
            }
            // check for top
            for ti in 0..j {
                if tree_map[i][ti] >= tree_map[i][j] {
                    covered += 1;
                    break;
                }
            }
            // check for bottom
            for bi in j+1..tree_map[i].len() {
                if tree_map[i][bi] >= tree_map[i][j] {
                    covered += 1;
                    break;
                }
            }

            if covered == 4 {
                covered_num += 1;
            }

        }
    }

    let visible_num = (tree_map.len() * tree_map[0].len()) as i32 - covered_num;

    return visible_num;
}

/**
 *
 */
fn part2(input: &Vec<String>) -> i32 {
    let mut max_visibility = 0;
    let tree_map = parse_input(input);

    for i in 1..tree_map.len()-1 {
        for j in 1..tree_map[0].len()-1 {
            // compute visibility score from top and multiply it with base visibility score
            let mut top_visibility = 0;
            for ti in (0..i).rev() {
                top_visibility += 1;
                if tree_map[ti][j] >= tree_map[i][j] {
                    break;
                }
            }

            // compute visibility score from bottom and multiply it with base visibility score
            let mut bottom_visibility = 0;
            for bi in i+1..tree_map.len() {
                bottom_visibility += 1;
                if tree_map[bi][j] >= tree_map[i][j] {
                    break;
                }
            }

            // compute visibility score from left and multiply it with base visibility score
            let mut left_visibility = 0;
            for li in (0..j).rev() {
                left_visibility += 1;
                if tree_map[i][li] >= tree_map[i][j] {
                    break;
                }
            }

            // compute visibility score from right and multiply it with base visibility score
            let mut right_visibility = 0;
            for ri in j+1..tree_map[0].len() {
                right_visibility += 1;
                if tree_map[i][ri] >= tree_map[i][j] {
                    break;
                }
            }

            let visibility = top_visibility * bottom_visibility * left_visibility * right_visibility;
            if visibility > max_visibility {
                max_visibility = visibility;
            }

        }
    }

    return max_visibility;
}

fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}