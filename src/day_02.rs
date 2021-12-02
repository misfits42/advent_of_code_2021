use regex::Regex;

enum Command {
    Forward{units: i64},
    Down{units: i64},
    Up{units: i64}
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Command> {
    let mut output: Vec<Command> = vec![];
    let forward_regex = Regex::new(r"^forward (\d+)$").unwrap();
    let down_regex = Regex::new(r"^down (\d+)$").unwrap();
    let up_regex = Regex::new(r"^up (\d+)$").unwrap();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Try to match forward regex
        if forward_regex.is_match(line) {
            let captures = forward_regex.captures(line).unwrap();
            let units_val = captures[1].parse::<i64>().unwrap();
            output.push(Command::Forward{units: units_val});
        // Try to match down regex
        } else if down_regex.is_match(line) {
            let captures = down_regex.captures(line).unwrap();
            let units_val = captures[1].parse::<i64>().unwrap();
            output.push(Command::Down{units: units_val});
        // Try to match up regex
        } else if up_regex.is_match(line) {
            let captures = up_regex.captures(line).unwrap();
            let units_val = captures[1].parse::<i64>().unwrap();
            output.push(Command::Up{units: units_val});
        // Line does not match an expected format - panic!
        } else {
            panic!("Day 2 - input line does not match an expected format!");
        }
    }
    return output;
}

#[aoc(day2, part1)]
fn solve_part_1(commands: &Vec<Command>) -> i64 {
    // Submarine starts at depth 0 and horizonal 0
    let mut x_pos: i64 = 0;
    let mut y_pos: i64 = 0;
    for command in commands {
        match command {
            Command::Forward{units} => {
                x_pos += units;
            },
            Command::Down{units} => {
                y_pos += units;
            },
            Command::Up{units} => {
                y_pos -= units;
            }
        }
    }
    return x_pos * y_pos;
}

#[aoc(day2, part2)]
fn solve_part_2(commands: &Vec<Command>) -> i64 {
    // Submarine starts with depth, horizonal and aim of 0
    let mut x_pos: i64 = 0;
    let mut y_pos: i64 = 0;
    let mut aim: i64 = 0;
    for command in commands {
        match command {
            Command::Forward{units} => {
                x_pos += units;
                y_pos += aim * units;
            },
            Command::Down{units} => {
                aim += units;
            },
            Command::Up{units} => {
                aim -= units;
            }
        }
    }
    return x_pos * y_pos;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d02_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day2.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(1524750, result);
    }

    #[test]
    fn test_d02_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day2.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1592426537, result);
    }
}
