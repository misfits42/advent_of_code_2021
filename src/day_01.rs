#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u64> {
    let mut output: Vec<u64> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value = line.parse::<u64>().unwrap();
        output.push(value);
    }
    return output;
}

#[aoc(day1, part1)]
fn solve_part_1(values: &Vec<u64>) -> u64 {
    // Initiate variables to track increase count and previous value
    let mut count: u64 = 0;
    for i in 1..values.len() {
        if values[i] > values[i - 1] {
            count += 1;
        }
    }
    return count;
}

#[aoc(day1, part2)]
fn solve_part_2(values: &Vec<u64>) -> u64 {
    // Initiate variable to track count and previous window value
    let mut count: u64 = 0;
    let mut previous: u64 = 0;
    let mut started = false;
    for i in 2..values.len() {
        let window_value = calculate_window_value(values, i);
        // Handle start case
        if !started {
            started = true;
            previous = window_value;
            continue;
        }
        // Check if current window value is greater than previous window value, then slide along
        if window_value > previous {
            count += 1;
        }
        previous = window_value;
    }
    return count;
}

/// Calculates the total value of the three-value window ending at the specified index.
/// 
/// Assumption: end_index is greater than or equal to 2.
fn calculate_window_value(values: &Vec<u64>, end_index: usize) -> u64 {
    if end_index < 2 {
        panic!("Day 1: bad end index value for sliding window calculation!");
    }
    let mut window_value: u64 = 0;
    for i in 0..3 {
        window_value += values[end_index - i];
    }
    return window_value;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d01_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day1.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(1266, result);
    }

    #[test]
    fn test_d01_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day1.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1217, result);
    }
}
