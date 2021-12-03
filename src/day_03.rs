use std::collections::HashMap;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut output: Vec<Vec<u8>> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let vals = line.chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
        output.push(vals);
    }
    return output;
}

#[aoc(day3, part1)]
fn solve_part_1(values: &Vec<Vec<u8>>) -> u64 {
    // Initialise the hash maps storing index frequency counts
    let mut tracker_zero: HashMap<usize, usize> = HashMap::new();
    let mut tracker_one: HashMap<usize, usize> = HashMap::new();
    for i in 0..12 {
        tracker_zero.insert(i, 0);
        tracker_one.insert(i, 0);
    }
    // Count the number of times 0 and 1 appear at each index
    for value in values {
        for i in 0..value.len() {
            if value[i] == 0 {
                *tracker_zero.get_mut(&i).unwrap() += 1;
            } else { // value[i] == 1
                *tracker_one.get_mut(&i).unwrap() += 1;
            }
        }
    }
    // Check which values are least and most comment for each index
    let mut gamma_binary = String::new();
    let mut epsilon_binary = String::new();
    for i in 0..12 {
        if tracker_zero.get(&i).unwrap() > tracker_one.get(&i).unwrap() {
            gamma_binary.push('0');
            epsilon_binary.push('1');
        } else {
            gamma_binary.push('1');
            epsilon_binary.push('0');
        }
    }
    // Convert gamma and epsilon rates from binary representations
    let gamma_rate = u64::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon_rate = u64::from_str_radix(&epsilon_binary, 2).unwrap();
    let power_rate = gamma_rate * epsilon_rate;
    return power_rate;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d03_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day3.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(3687446, result);
    }
}
