use std::collections::HashMap;

use regex::Regex;

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<String> {
    let mut snailfish_numbers: Vec<String> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        snailfish_numbers.push(line.to_string());
    }
    return snailfish_numbers;
}

fn calculate_magnitude_sum(snailfish_number: &String) -> u64 {
    let pair_regex = Regex::new(r"(\[\d+,\d+\])").unwrap();
    let pair_value_extract_regex = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut magnitude_sum = snailfish_number.to_string();
    loop {
        // Find all pairs that have not been converted to single value
        let captures = {
            let caps = pair_regex.captures(&magnitude_sum);
            if caps.is_none() {
                break
            } else {
                caps.unwrap()
            }
        };
        // Determine sub magnitude sum for all remaining pairs
        let mut replace_pairs: HashMap<String, String> = HashMap::new();
        for i in 1..captures.len() {
            if replace_pairs.contains_key(&captures[i]) {
                continue;
            }
            let pair_captures = pair_value_extract_regex.captures(&captures[i]).unwrap();
            let left = pair_captures[1].parse::<u64>().unwrap();
            let right = pair_captures[2].parse::<u64>().unwrap();
            let magnitude = left * 3 + right * 2;
            replace_pairs.insert(captures[i].to_string(), format!("{}", magnitude));
        }
        // Replace pairs with magnitude sums
        for (original, new) in replace_pairs.iter() {
            magnitude_sum = magnitude_sum.replace(original, new);
        }
    }
    return magnitude_sum.parse::<u64>().unwrap();
}

fn tokenise_snailfish_number(snailfish_number: &String) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let input_chars = snailfish_number.chars().collect::<Vec<char>>();
    let mut cursor: usize = 0;
    loop {
        if cursor >= input_chars.len() {
            break;
        }
        if input_chars[cursor].is_digit(10) {
            let mut delta = 1;
            let mut value_string = String::from(input_chars[cursor]);
            loop {
                if !input_chars[cursor + delta].is_digit(10) {
                    break;
                } else {
                    value_string.push(input_chars[cursor + delta]);
                    delta += 1;
                }
            }
            output.push(value_string);
            cursor += delta;
        } else {
            output.push(String::from(input_chars[cursor]));
            cursor += 1;
        }
    }
    return output;
}

#[aoc(day18, part1)]
fn solve_part_1(snailfish_numbers: &Vec<String>) -> u64 {
    let mut result = snailfish_numbers[0].to_string();
    for i in 1..snailfish_numbers.len() {
        result = format!("[{},{}]", result, snailfish_numbers[i]);
        // Keep applying reduction actions until no further reductions required on running result
        let mut cursor: usize = 0;
        let mut tokens = tokenise_snailfish_number(&result);
        let mut depth = 0;
        let mut explode_check_finished = false;
        loop {
            if cursor >= tokens.len() {
                if explode_check_finished {
                    // Reduction finished if end reached with explode and split checks complete
                    break;
                } else {
                    // Explode check finished - onto splits
                    explode_check_finished = true;
                    cursor = 0;
                    depth = 0;
                }
            }
            match tokens[cursor].as_str() {
                "[" => {
                    depth += 1;
                    cursor += 1;
                },
                "]" => {
                    depth -= 1;
                    cursor += 1;
                },
                "," => {
                    cursor += 1;
                },
                _ => {
                    // Numeric token
                    let value = tokens[cursor].parse::<u64>().unwrap();
                    // Explode
                    if depth > 4 {
                        // Add left to previous numberic token
                        let mut left_cursor = cursor;
                        let left_value = value;
                        loop {
                            if left_cursor == 0 {
                                break;
                            }
                            left_cursor -= 1;
                            // Check if numeric token to left is found
                            let target_base = tokens[left_cursor].parse::<u64>();
                            if target_base.is_ok() {
                                let target_base = target_base.unwrap();
                                let sum = target_base + left_value;
                                tokens[left_cursor] = sum.to_string();
                                break;
                            }
                        }
                        // Add right to next numeric token
                        let right_value = tokens[cursor + 2].parse::<u64>().unwrap();
                        let mut right_cursor = cursor + 2;
                        loop {
                            if right_cursor >= tokens.len() - 1 {
                                break;
                            }
                            right_cursor += 1;
                            // Check if numberic token to right is found
                            let target_base = tokens[right_cursor].parse::<u64>();
                            if target_base.is_ok() {
                                let target_base = target_base.unwrap();
                                let sum = target_base + right_value;
                                tokens[right_cursor] = sum.to_string();
                                break;
                            }
                        }
                        // Replace exploded pair with 0
                        tokens[cursor - 1] = String::from("0");
                        for i in 0..4 {
                            tokens[cursor + i] = String::from("");
                        }
                        // Go back to the beginning and re-check entire result string
                        cursor = 0;
                        depth = 0;
                        tokens = tokenise_snailfish_number(&tokens.join(""));
                    // Split
                    } else if value >= 10 {
                        if !explode_check_finished {
                            cursor += 1;
                            continue;
                        }
                        // println!("## Spliting at cursor {}", cursor);
                        let new_pair_left = value / 2;
                        let new_pair_right = (value + 1) / 2;
                        let new_pair = format!("[{},{}]", new_pair_left, new_pair_right);
                        tokens[cursor] = new_pair;
                        // Start back at beginning and re-check entire result string, explodes first
                        cursor = 0;
                        depth = 0;
                        tokens = tokenise_snailfish_number(&tokens.join(""));
                        explode_check_finished = false;
                    } else {
                        cursor += 1;
                    }
                }
            }
        }
        result = tokens.join("");
    }
    // Calculate magnitude of result
    return calculate_magnitude_sum(&result);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d18_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day18.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(4435, result);
    }
}