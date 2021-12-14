use std::collections::HashMap;

#[aoc_generator(day14)]
fn parse_input(input: &str) -> (String, HashMap<String, char>) {
    let mut insertion_rules: HashMap<String, char> = HashMap::new();
    let mut polymer_template = String::new();
    let mut check_polymer_template = true;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            check_polymer_template = false;
            continue;
        }
        if check_polymer_template {
            polymer_template = line.to_string();
        } else {
            let split = line.split(" -> ").collect::<Vec<&str>>();
            insertion_rules.insert(split[0].to_string(), split[1].chars().next().unwrap());
        }
    }
    return (polymer_template, insertion_rules);
}

#[aoc(day14, part1)]
fn solve_part_1(polymer_data: &(String, HashMap<String, char>)) -> u64 {
    let mut polymer_template = polymer_data.0.clone();
    let insertion_rules = &polymer_data.1;
    // Apply 10 iterations of insertion pair processing
    for _ in 0..10 {
        polymer_template = apply_polymer_insertion_rules(&polymer_template, &insertion_rules);
    }
    // Determine difference between maximum and minimum count of elements
    return calculate_character_max_min_difference(&polymer_template);
}

#[aoc(day14, part2)]
fn solve_part_2(polymer_data: &(String, HashMap<String, char>)) -> u64 {
    // Count number of times each distinct element pair appears
    let polymer_chars = polymer_data.0.chars().collect::<Vec<char>>();
    let mut polymer_pairs: HashMap<String, u64> = HashMap::new();
    for i in 1..polymer_chars.len() {
        let current_pair = {
            let mut s = String::new();
            s.push(polymer_chars[i - 1]);
            s.push(polymer_chars[i]);
            s
        };
        *polymer_pairs.entry(current_pair).or_insert(0) += 1;
    }
    let insertion_rules = &polymer_data.1;
    // Apply 40 iterations of insertion pair processing
    for _ in 0..40 {
        let mut new_polymer_pairs: HashMap<String, u64> = HashMap::new();
        for (pair, count) in polymer_pairs.into_iter() {
            if insertion_rules.contains_key(&pair) {
                let insertion_char = *insertion_rules.get(&pair).unwrap();
                let old_pair_chars = pair.chars().collect::<Vec<char>>();
                let mut new_left_pair = String::new();
                let mut new_right_pair = String::new();
                new_left_pair.push(old_pair_chars[0]);
                new_left_pair.push(insertion_char);
                new_right_pair.push(insertion_char);
                new_right_pair.push(old_pair_chars[1]);
                *new_polymer_pairs.entry(new_left_pair).or_insert(0) += count;
                *new_polymer_pairs.entry(new_right_pair).or_insert(0) += count;
            } else {
                *new_polymer_pairs.entry(pair).or_insert(0) += count;
            }
        }
        polymer_pairs = new_polymer_pairs;
    }
    // Determine character count
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    for (pair, count) in polymer_pairs.iter() {
        let pair_chars = pair.chars().collect::<Vec<char>>();
        *char_counts.entry(pair_chars[0]).or_insert(0) += count;
        *char_counts.entry(pair_chars[1]).or_insert(0) += count;
    }
    // Adjust figures for start and end char of original polymer template
    for (c, count) in char_counts.iter_mut() {
        *count /= 2;
        if *c == polymer_chars[0] || *c == polymer_chars[polymer_chars.len() - 1] {
            *count += 1;
        }
    }
    let max = char_counts.values().max().unwrap();
    let min = char_counts.values().min().unwrap();
    return max - min;
}

/// Applies one iteration of the insertion rules to the given polymer template. Return value is the
/// resulting polymer template after applying insertion rules.
fn apply_polymer_insertion_rules(polymer_template: &String, insertion_rules: &HashMap<String, char>) -> String {
    let mut new_polymer_template = String::new();
    let polymer_chars = polymer_template.chars().collect::<Vec<char>>();
    for i in 1..polymer_chars.len() {
        // Determine the current character pair within sliding window
        let current_pair = {
            let mut s = String::new();
            s.push(polymer_chars[i - 1]);
            s.push(polymer_chars[i]);
            s
        };
        // Check if we have a matching insertion pair rule
        if i == 1 {
            new_polymer_template.push(polymer_chars[i - 1]);
        }
        if insertion_rules.contains_key(&current_pair) {
            new_polymer_template.push(*insertion_rules.get(&current_pair).unwrap());
        }
        new_polymer_template.push(polymer_chars[i]);
    }
    return new_polymer_template;
}

/// Calculates the difference in the counts for the most and least common characters in the given
/// polymer template.
fn calculate_character_max_min_difference(polymer_template: &String) -> u64 {
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    for c in polymer_template.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    let max = char_counts.values().max().unwrap();
    let min = char_counts.values().min().unwrap();
    return max - min;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d14_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day14.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(2768, result);
    }

    #[test]
    fn test_d14_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day14.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(2914365137499, result);
    }
}
