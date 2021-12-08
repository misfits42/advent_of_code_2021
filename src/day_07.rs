#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i64> {
    let mut output = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    output.sort();
    return output;
}

#[aoc(day7, part1)]
fn solve_part_1(initial_pos: &Vec<i64>) -> i64 {
    let mut min_fuel: i64 = i64::MAX;
    // Assuming initial positions have already been sorted
    for target in initial_pos[0]..=initial_pos[initial_pos.len() - 1] {
        // Calculate fuel used to moved to current target position
        let mut fuel_used = 0;
        for pos in initial_pos.iter() {
            fuel_used += (target - pos).abs();
        }
        if fuel_used < min_fuel {
            min_fuel = fuel_used;
        }
    }
    return min_fuel;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    // Test cases go here
    #[test]
    fn test_d07_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day7.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(347011, result);
    }
}
