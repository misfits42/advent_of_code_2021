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
}
