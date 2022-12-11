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
 * Parse the input file into a vector of monkeys' items, operations, and destinations.
 */
fn parse_input(input: &Vec<String>) -> Vec<(Vec<i64>, String, String, String, i64, i64, i64)> {
    // register the monkeys' operations
    // each operation is a tuple of
    // (operation, operand 1, operand 2, divisible by, dst1, dst2)
    let mut monkeys: Vec<(Vec<i64>, String, String, String, i64, i64, i64)> = Vec::new();
    let mut i = 0;
    while i < input.len() {
        if input[i].starts_with("Monkey") {
            let mut items = Vec::new();

            let (operation, operand1, operand2, divisible_by, dst1, dst2);
            let (mut l_bracket, mut r_bracket);

            // parse items
            i += 1;
            let mut cursor = "  Starting items: ".len();
            while cursor < input[i].len() {
                if input[i].chars().nth(cursor).unwrap() == ',' {
                    cursor += 2;
                } else {
                    let item_num = input[i][cursor..cursor + 2].parse::<i64>().unwrap();
                    items.push(item_num);
                    cursor += 2;
                }
            }

            // parse operation type
            i += 1;
            if input[i].contains("*") {
                operation = "*";
            } else if input[i].contains("+") {
                operation = "+";
            } else {
                panic!("Unknown operation type");
            }

            // parse operand 1
            l_bracket = "  Operation: new = ".len();
            r_bracket = input[i].find(operation).unwrap() - 1;
            operand1 = &input[i][l_bracket..r_bracket];

            // parse operand 2
            l_bracket = input[i].find(operation).unwrap() + 2;
            r_bracket = input[i].len();
            operand2 = &input[i][l_bracket..r_bracket];

            // parse divisible by
            i += 1;
            l_bracket = "  Test: divisible by ".len();
            r_bracket = input[i].len();
            divisible_by = input[i][l_bracket..r_bracket].parse::<i64>().unwrap();

            // parse dst1
            i += 1;
            l_bracket = "    If true: throw to monkey ".len();
            r_bracket = input[i].len();
            dst1 = input[i][l_bracket..r_bracket].parse::<i64>().unwrap();

            // parse dst2
            i += 1;
            l_bracket = "    If false: throw to monkey ".len();
            r_bracket = input[i].len();
            dst2 = input[i][l_bracket..r_bracket].parse::<i64>().unwrap();

            // push the operation to the monkey's operations
            monkeys.push((items, operation.to_string(), operand1.to_string(), operand2.to_string(), divisible_by, dst1, dst2));
        } else {
            i += 1;
        }
    }

    return monkeys;
}

/**
 * Run the simulation for part 1.
 *
 * Each monkey does its work, and the items are passed to the next monkey.
 * The simulation runs for 20 rounds.
 * The product of the two highest inspection counts is returned.
 */
fn part1(input: &Vec<String>) -> i32 {
    let monkeys = parse_input(input);

    let mut inspects = vec![0; monkeys.len()];

    // copy the items
    let mut items = Vec::new();
    for i in 0..monkeys.len() {
        items.push(monkeys[i].0.clone());
    }

    for _ in 0..20 {
        // each monkey does its work
        for i in 0..monkeys.len() {
            let (operation, operand1, operand2, divisible_by, dst1, dst2) = (&monkeys[i].1, &monkeys[i].2, &monkeys[i].3, monkeys[i].4, monkeys[i].5, monkeys[i].6);

            // println!("Monkey {} inspects {:?}", i, items[i]);

            while items[i].len() > 0 {
                inspects[i] += 1;

                let item = items[i][0];
                items[i].remove(0);

                // println!("Monkey {} holds {:?}", i, items[i]);

                let (op1, op2);
                if operand1.contains("old") {
                    op1 = item as i64;
                } else {
                    op1 = operand1.parse::<i64>().unwrap();
                }

                if operand2.contains("old") {
                    op2 = item as i64;
                } else {
                    op2 = operand2.parse::<i64>().unwrap();
                }

                let worry_level = match operation.as_str() {
                    "*" => op1 * op2,
                    "+" => op1 + op2,
                    _ => panic!("Unknown operation type"),
                };

                if (worry_level as i64 / 3) % (divisible_by as i64) == 0 {
                    items[dst1 as usize].push(worry_level as i64 / 3);

                } else {
                    items[dst2 as usize].push(worry_level as i64 / 3);
                }
            }
        }
    }

    // find the two largest inspect counts
    let mut max1 = 0;
    let mut max2 = 0;
    for i in 0..inspects.len() {
        if inspects[i] > max1 {
            max2 = max1;
            max1 = inspects[i];
        } else if inspects[i] > max2 {
            max2 = inspects[i];
        }
    }

    return max1 * max2;
}

/**
 * Run the simulation for part 2.
 *
 * Each monkey does its work, and the items are passed to the next monkey.
 * The simulation runs for 10000 rounds.
 * The product of the two highest inspection counts is returned.
 *
 * To make sure the worry levels are not too large, the worry levels are mod the product of a series of primes.
 */
fn part2(input: &Vec<String>) -> i64 {
    let monkeys = parse_input(input);

    let mut inspects = vec![0; monkeys.len()];

    // copy the items
    let mut items = Vec::new();
    for i in 0..monkeys.len() {
        items.push(monkeys[i].0.clone());
    }

    for _ in 0..10000 {
        // each monkey does its work
        for i in 0..monkeys.len() {
            let (operation, operand1, operand2, divisible_by, dst1, dst2) = (&monkeys[i].1, &monkeys[i].2, &monkeys[i].3, monkeys[i].4, monkeys[i].5, monkeys[i].6);

            // println!("Monkey {} inspects {:?}", i, items[i]);

            while items[i].len() > 0 {
                inspects[i] += 1;

                let item = items[i][0];
                items[i].remove(0);

                // println!("Monkey {} holds {:?}", i, items[i]);

                let (op1, op2);
                if operand1.contains("old") {
                    op1 = item as i64;
                } else {
                    op1 = operand1.parse::<i64>().unwrap();
                }

                if operand2.contains("old") {
                    op2 = item as i64;
                } else {
                    op2 = operand2.parse::<i64>().unwrap();
                }

                let worry_level = match operation.as_str() {
                    "*" => op1 * op2,
                    "+" => op1 + op2,
                    _ => panic!("Unknown operation type"),
                };

                let primes_prod: i64 = 2 * 3 * 5 * 7 * 9 * 11 * 13 * 17 * 19 * 23;
                if worry_level % (divisible_by as i64) == 0 {
                    items[dst1 as usize].push(worry_level % primes_prod);

                } else {
                    items[dst2 as usize].push(worry_level % primes_prod);
                }
            }
        }
    }

    // find the two largest inspect counts
    let mut max1 = 0;
    let mut max2 = 0;
    for i in 0..inspects.len() {
        if inspects[i] > max1 {
            max2 = max1;
            max1 = inspects[i];
        } else if inspects[i] > max2 {
            max2 = inspects[i];
        }
    }

    return max1 as i64 * max2 as i64;
}

fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}