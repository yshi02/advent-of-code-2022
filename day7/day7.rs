use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}


/**
 * Given a list of commands and their outputs,
 * returns a hashmap with the path/to/a/file as the key and the size as the value.
 */
fn parse_input(input: &Vec<String>) -> HashMap<String, i32> {
    let mut dirs = Vec::new();
    let mut history = Vec::new();
    let mut dict: HashMap<String, i32> = HashMap::new();

    for i in 0..input.len() {
        let line = &input[i];

        // deals with directories
        if line.starts_with("$ cd ") {
            let dir = line.split(" ").collect::<Vec<&str>>()[2];
            if dir == ".." {
                history.pop();
            } else {
                history.push(dir.to_string());
            }

        // deal with dirs and files in current directory
        } else if line.starts_with("$ ls") {
            let mut j = i;

            // gets the next few lines until it hits a new command
            loop {
                // if hits EOF
                if j == input.len() - 1 {
                    break;
                }

                j += 1;
                let line = &input[j];

                // hits a new command
                if line.starts_with("$") {
                    break;

                // deals with directories, only adds it to a list of directories
                } else if line.starts_with("dir") {
                    let dir = line.split(" ").collect::<Vec<&str>>()[1];
                    let mut path = String::new();
                    for j in 0..history.len() {
                        path.push_str(&history[j]);
                        path.push_str("/");
                    }
                    path.push_str(dir);
                    path.push_str("/");
                    dirs.push(path);

                // deals with files, adds it to a hashmap with the path as the key and the size as the value
                } else {
                    let file_size = line.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                    let file_name = line.split(" ").collect::<Vec<&str>>()[1];
                    let mut path = String::new();
                    for j in 0..history.len() {
                        path.push_str(&history[j]);
                        path.push_str("/");
                    }
                    path.push_str(file_name);
                    dict.insert(path, file_size);
                }
            }
        }
    }

    // now we have all directory paths, file paths, and file sizes
    // it's time to figure out directory sizes
    for dir in dirs {
        let mut size = 0;
        for (key, value) in &dict {
            if key.starts_with(&dir) {
                size += value;
            }
        }
        dict.insert(dir, size);
    }

    // also add the root directory (//) to the dict
    let mut size = 0;
    for (key, value) in &dict {
        if key.starts_with("//") && !key.ends_with("/") {
            size += value;
        }
    }
    dict.insert("//".to_string(), size);

    // print the dict
    // for (key, value) in &dict {
    //     println!("{}: {}", key, value);
    // }

    return dict;
}

/**
 * Given a list of commands and their outputs,
 * returns the sum of all directories with size <= 100000.
 */
fn part1(input: &Vec<String>) -> i32 {
    let dict = parse_input(input);

    // sums all directories with size <= 100000
    let mut sum = 0;
    for (key, value) in &dict {
        if (key.ends_with("/")) && (*value <= 100000) {
            sum += value;
        }
    }

    return sum;
}

/**
 * Given a list of commands and their outputs,
 * returns the size of the smallest directory which, when freed,
 * would allow the total space to be >= 30000000.
 */
fn part2(input: &Vec<String>) -> i32 {
    let dict = parse_input(input);

    let total_space = 70000000;
    let goal = 30000000;
    let want = goal - (total_space - dict["//"]);

    // find the smallest directory that is >= want
    let mut smallest = i32::MAX;
    for (key, value) in &dict {
        if (key.ends_with("/")) && (*value >= want) && (*value < smallest) {
            smallest = *value;
        }
    }

    return smallest;
}

fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}