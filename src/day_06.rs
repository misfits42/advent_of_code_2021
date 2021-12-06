use std::collections::HashMap;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> HashMap<u64, u64> {
    // Track remaining timer as key and number of occurrences as value
    let mut output: HashMap<u64, u64> = HashMap::new();
    let initial_ages = input.lines().next().unwrap().split(",").map(|x| x.parse::<u64>().unwrap());
    for age in initial_ages {
        *output.entry(age).or_insert(0) += 1;
    }
    return output;
}

#[aoc(day6, part1)]
fn solve_part_1(initial_fish: &HashMap<u64, u64>) -> u64 {
    let mut current_fish = initial_fish.clone();
    // Simulate for 80 days
    for _ in 0..80 {
        current_fish = conduct_single_turn(&current_fish);
    }
    return current_fish.values().sum();
}

#[aoc(day6, part2)]
fn solve_part_2(initial_fish: &HashMap<u64, u64>) -> u64 {
    let mut current_fish = initial_fish.clone();
    // Simulate for 256 days
    for _ in 0..256 {
        current_fish = conduct_single_turn(&current_fish);
    }
    return current_fish.values().sum();
}

/// Conducts a single turn of the lanternfish simulation. Input is the fish population state before
/// the turn, and return value is the population state after the turn.
fn conduct_single_turn(current_fish: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_fish: HashMap<u64, u64> = HashMap::new();
    for (k, v) in current_fish.iter() {
        match k {
            // Handle case where new fish are born
            0 => {
                new_fish.insert(8, *v);
                // Handle edge case if we have already seen the fish go down to 6 timer
                if new_fish.contains_key(&6) {
                    *new_fish.get_mut(&6).unwrap() += v;
                } else {
                    new_fish.insert(6, *v);
                }
            },
            // Handle all other cases (1-8)
            _ => {
                let new_timer = k - 1;
                // This is required for the end-case of reborn fish and first-aging fish
                if new_fish.contains_key(&new_timer) {
                    *new_fish.get_mut(&new_timer).unwrap() += v;
                } else {
                    new_fish.insert(new_timer, *v);
                }
            }
        }
    }
    return new_fish;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d06_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day6.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(366057, result);
    }

    #[test]
    fn test_d06_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day6.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1653559299811, result);
    }
}
