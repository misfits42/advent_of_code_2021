#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut output: Vec<(Vec<String>, Vec<String>)> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Separate the unique signal patterns and output values from input line
        let mut split = line.split(" | ");
        let signal_patterns = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let output_values = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        output.push((signal_patterns, output_values));
    }
    return output;
}

#[aoc(day8, part1)]
fn solve_part_1(entries: &Vec<(Vec<String>, Vec<String>)>) -> u64 {
    // Keep track of how many times a 1, 4, 7 or 8 digit appears in output values
    let mut count = 0;
    for (_signal_patterns, output_values) in entries {
        for value in output_values {
            let len = value.len();
            if len == 2 || len == 4 || len == 3 || len == 7 {
                count += 1;
            }
        }
    }
    return count;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d08_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day8.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(532, result);
    }
}
