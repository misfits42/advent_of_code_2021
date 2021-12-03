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
    // Calculate the binary representations of the gamma and epsilon rates
    let (gamma_binary, epsilon_binary) = calculate_gamma_and_epsilon_binaries(values);
    // Convert gamma and epsilon rates from binary representations
    let gamma_rate = u64::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon_rate = u64::from_str_radix(&epsilon_binary, 2).unwrap();
    let power_rate = gamma_rate * epsilon_rate;
    return power_rate;
}

fn calculate_index_value_frequencies(values: &Vec<Vec<u8>>) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
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
    return (tracker_zero, tracker_one);
}

fn calculate_gamma_and_epsilon_binaries(values: &Vec<Vec<u8>>) -> (String, String) {
    let (tracker_zero, tracker_one) = calculate_index_value_frequencies(values);
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
    return (gamma_binary, epsilon_binary);
}

fn calculate_o2_generator_rating(values: &Vec<Vec<u8>>) -> u64 {// Determine oxygen generator rating
    // Keep track of values remaining and current index
    let mut oxygen_values = values.clone();
    let mut oxygen_i = 0;
    loop {
        // Stop when we have only one value remaining - this is our oxygen generator rating
        if oxygen_values.len() == 1 {
            break;
        }
        let mut new_values: Vec<Vec<u8>> = vec![];
        // Recalculate index value frequencies
        let (tracker_zero, tracker_one) = calculate_index_value_frequencies(&oxygen_values);
        // Target value for current index is most common value, or 1 in event of tie
        let target = {
            if tracker_one.get(&oxygen_i).unwrap() >= tracker_zero.get(&oxygen_i).unwrap() {
                1
            } else {
                0
            }
        };
        // Keep only values with the target value at the current index
        for oxygen_value in oxygen_values {
            if oxygen_value[oxygen_i] == target {
                new_values.push(oxygen_value.clone());
            }
        }
        // Replace search values with filtered values and increase index
        oxygen_values = new_values;
        oxygen_i += 1;
    }
    // Convert the rating from binary to decimal form
    let mut oxygen_gen_string = String::new();
    for i in 0..12 {
        if oxygen_values[0][i] == 0 {
            oxygen_gen_string.push('0');
        } else {
            oxygen_gen_string.push('1');
        }
    }
    return u64::from_str_radix(&oxygen_gen_string, 2).unwrap();
}

fn calculate_co2_scrubber_rating(values: &Vec<Vec<u8>>) -> u64 {
    // Determine CO2 scrubber rating
    let mut co2_values = values.clone();
    let mut co2_i = 0;
    loop {
        // Stop when we have only one value remaining - this is our CO2 scrubber rating
        if co2_values.len() == 1 {
            break;
        }
        let mut new_values: Vec<Vec<u8>> = vec![];
        // Recalculate index value frequencies
        let (tracker_zero, tracker_one) = calculate_index_value_frequencies(&co2_values);
        // Target value for current index is least common value, or 0 in event of tie
        let target = {
            if tracker_zero.get(&co2_i).unwrap() <= tracker_one.get(&co2_i).unwrap() {
                0
            } else {
                1
            }
        };
        // Keep only the values with the target value at current index
        for co2_value in co2_values {
            if co2_value[co2_i] == target {
                new_values.push(co2_value.clone());
            }
        }
        co2_values = new_values;
        co2_i += 1;
    }
    // Convert the rating from binary to decimal form
    let mut co2_scrubber_string = String::new();
    for i in 0..12 {
        if co2_values[0][i] == 0 {
            co2_scrubber_string.push('0');
        } else {
            co2_scrubber_string.push('1');
        }
    }
    return u64::from_str_radix(&co2_scrubber_string, 2).unwrap();
}

#[aoc(day3, part2)]
fn solve_part_2(values: &Vec<Vec<u8>>) -> u64 {
    let o2_generator_rating = calculate_o2_generator_rating(values);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(values);
    return o2_generator_rating * co2_scrubber_rating;
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

    #[test]
    fn test_d03_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day3.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(4406844, result);
    }
}
