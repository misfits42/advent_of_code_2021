use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

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
            .map(|x| x.chars().sorted().collect::<String>())
            .collect::<Vec<String>>();
        let output_values = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.chars().sorted().collect::<String>())
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

#[aoc(day8, part2)]
fn solve_part_2(entries: &Vec<(Vec<String>, Vec<String>)>) -> u64 {
    // Add up total of displayed output values
    let mut total = 0;
    let canonical_digits: HashMap<char, String> = generate_canonical_digit_displays();
    // Check each entry
    for (signal_combos, output_values) in entries {
        // Determine mapping of input wires to display segments
        // Add possible characters from the unique output digits 1, 4, 7 and 8
        let mut possibles = initialise_wire_possibles_map();
        add_unique_digits_to_possibles(signal_combos, &mut possibles);
        // Determine possible signal wires to output segments for the 5-segment digits
        add_5_segment_digits_to_possibles(signal_combos, &mut possibles);
        // Determine possible signal wires to output segments for the 6-segment digits
        add_6_segment_digits_to_possibles(signal_combos, &mut possibles);
        // Determine intersection of possible signal wires for each output segment
        // Actual output segment is mapped to the corresponding input signal wire
        let signal_wire_map = generate_signal_wire_map(&possibles);
        // Map output digit appearance to the digit itself
        let mut digit_appearances: HashMap<String, char> = HashMap::new();
        for c in "0123456789".chars() {
            let mut app = String::new();
            for digit_c in canonical_digits.get(&c).unwrap().chars() {
                app.push(*signal_wire_map.get(&digit_c).unwrap());
            }
            app = app.chars().sorted().collect::<String>();
            digit_appearances.insert(app, c);
        }
        // Now we have outputs mapped to actual digits - convert display to integer value
        let mut output_display_string = String::new();
        for digit in output_values {
            output_display_string.push(*digit_appearances.get(digit).unwrap());
        }
        total += output_display_string.parse::<u64>().unwrap();
    }
    return total;
}

/// Adds the signal wires for the digits with unique numbers of segments to the record of possible
/// wire-segment mapping.
fn add_unique_digits_to_possibles(
    signal_combos: &Vec<String>,
    possibles: &mut HashMap<char, Vec<HashSet<char>>>,
) {
    // Possibles: signal digit 1
    let signal_1_set = {
        let mut output = "";
        for combo in signal_combos {
            if combo.len() == 2 {
                output = combo;
                break;
            }
        }
        output.chars().collect::<HashSet<char>>()
    };
    possibles.get_mut(&'c').unwrap().push(signal_1_set.clone());
    possibles.get_mut(&'f').unwrap().push(signal_1_set.clone());
    // Possibles: signal digit 4
    let signal_4_set = {
        let mut output = "";
        for combo in signal_combos {
            if combo.len() == 4 {
                output = combo;
                break;
            }
        }
        output.chars().collect::<HashSet<char>>()
    };
    possibles.get_mut(&'b').unwrap().push(signal_4_set.clone());
    possibles.get_mut(&'c').unwrap().push(signal_4_set.clone());
    possibles.get_mut(&'d').unwrap().push(signal_4_set.clone());
    possibles.get_mut(&'f').unwrap().push(signal_4_set.clone());
    // Possibles: signal digit 7
    let signal_7_set = {
        let mut output = "";
        for combo in signal_combos {
            if combo.len() == 3 {
                output = combo;
                break;
            }
        }
        output.chars().collect::<HashSet<char>>()
    };
    possibles.get_mut(&'a').unwrap().push(signal_7_set.clone());
    possibles.get_mut(&'c').unwrap().push(signal_7_set.clone());
    possibles.get_mut(&'f').unwrap().push(signal_7_set.clone());
    // Possibles: signal digit 8
    let signal_8_set = {
        let mut output = "";
        for combo in signal_combos {
            if combo.len() == 7 {
                output = combo;
                break;
            }
        }
        output.chars().collect::<HashSet<char>>()
    };
    possibles.get_mut(&'a').unwrap().push(signal_8_set.clone());
    possibles.get_mut(&'b').unwrap().push(signal_8_set.clone());
    possibles.get_mut(&'c').unwrap().push(signal_8_set.clone());
    possibles.get_mut(&'d').unwrap().push(signal_8_set.clone());
    possibles.get_mut(&'e').unwrap().push(signal_8_set.clone());
    possibles.get_mut(&'f').unwrap().push(signal_8_set.clone());
    possibles.get_mut(&'g').unwrap().push(signal_8_set.clone());
}

/// Adds the signal wire possibilities for the 5-segment digits (2, 3 and 5) to the record of
/// possible wire-segment mapping.
fn add_5_segment_digits_to_possibles(
    signal_combos: &Vec<String>,
    possibles: &mut HashMap<char, Vec<HashSet<char>>>,
) {
    // Find the signal patterns corresponding to the 5-segment digits
    let mut signals_5_segment: Vec<String> = vec![];
    for combo in signal_combos {
        if combo.len() == 5 {
            signals_5_segment.push(combo.to_string());
        }
    }
    // Determine the signal wire possibilities for each display segment by determining overlaps
    let mut occurrence_1_set: HashSet<char> = HashSet::new();
    let mut occurrence_2_set: HashSet<char> = HashSet::new();
    let mut occurrence_3_set: HashSet<char> = HashSet::new();
    for i in 0..3 {
        for c in signals_5_segment[i].chars() {
            let others = {
                if i == 0 {
                    (1, 2)
                } else if i == 1 {
                    (0, 2)
                } else {
                    (0, 1)
                }
            };
            let mut count = 1;
            if signals_5_segment[others.0].contains(c) {
                count += 1;
            }
            if signals_5_segment[others.1].contains(c) {
                count += 1;
            }
            if count == 1 {
                occurrence_1_set.insert(c);
            } else if count == 2 {
                occurrence_2_set.insert(c);
            } else {
                occurrence_3_set.insert(c);
            }
        }
    }
    // Add to possible wire-segment map - e.g. segment 'a' sees three overlapping digits
    possibles.get_mut(&'a').unwrap().push(occurrence_3_set.clone());
    possibles.get_mut(&'b').unwrap().push(occurrence_1_set.clone());
    possibles.get_mut(&'c').unwrap().push(occurrence_2_set.clone());
    possibles.get_mut(&'d').unwrap().push(occurrence_3_set.clone());
    possibles.get_mut(&'e').unwrap().push(occurrence_1_set.clone());
    possibles.get_mut(&'f').unwrap().push(occurrence_2_set.clone());
    possibles.get_mut(&'g').unwrap().push(occurrence_3_set.clone());
}

/// Adds the signal wire possibilities for the 6-segment digits (0, 6, 9) to the record of possible
/// wire-segment mapping.
fn add_6_segment_digits_to_possibles(
    signal_combos: &Vec<String>,
    possibles: &mut HashMap<char, Vec<HashSet<char>>>,
) {
    // Determine the signal patterns corresponding to the 6-segment digits
    let mut signals_6_segment: Vec<String> = vec![];
    for combo in signal_combos {
        if combo.len() == 6 {
            signals_6_segment.push(combo.to_string());
        }
    }
    // Determine the signal wire possibilities by determining the overlaps for each segment
    let mut occurrence_1_set: HashSet<char> = HashSet::new();
    let mut occurrence_2_set: HashSet<char> = HashSet::new();
    let mut occurrence_3_set: HashSet<char> = HashSet::new();
    for i in 0..3 {
        for c in signals_6_segment[i].chars() {
            let others = {
                if i == 0 {
                    (1, 2)
                } else if i == 1 {
                    (0, 2)
                } else {
                    (0, 1)
                }
            };
            let mut count = 1;
            if signals_6_segment[others.0].contains(c) {
                count += 1;
            }
            if signals_6_segment[others.1].contains(c) {
                count += 1;
            }
            if count == 1 {
                occurrence_1_set.insert(c);
            } else if count == 2 {
                occurrence_2_set.insert(c);
            } else {
                occurrence_3_set.insert(c);
            }
        }
    }
    // Add to possible wire-segment map - e.g. segment 'a' sees three digits overlap
    possibles.get_mut(&'a').unwrap().push(occurrence_3_set.clone());
    possibles.get_mut(&'b').unwrap().push(occurrence_3_set.clone());
    possibles.get_mut(&'c').unwrap().push(occurrence_2_set.clone());
    possibles.get_mut(&'d').unwrap().push(occurrence_2_set.clone());
    possibles.get_mut(&'e').unwrap().push(occurrence_2_set.clone());
    possibles.get_mut(&'f').unwrap().push(occurrence_3_set.clone());
    possibles.get_mut(&'g').unwrap().push(occurrence_3_set.clone());
}

/// Processes the sets of possible signal wires for each segment to determine what signal wire
/// corresponds to each actual segment.
fn generate_signal_wire_map(possibles: &HashMap<char, Vec<HashSet<char>>>) -> HashMap<char, char> {
    let mut overlaps: HashMap<char, HashSet<char>> = HashMap::new();
        for key in possibles.keys() {
            let sets = possibles.get(&key).unwrap();
            let mut output_set: HashSet<char> = HashSet::new();
            for i in 0..sets.len() {
                if i == 0 {
                    output_set = sets[i].clone();
                    continue;
                }
                output_set = output_set.intersection(&sets[i]).map(|x| *x).collect::<HashSet<char>>();
            }
            overlaps.insert(*key, output_set);
        }
        // Remove additional values from overlaps so each output segment has one signal wire
        let mut unique_output_segments: HashSet<char> = HashSet::new();
        for (_key, value) in overlaps.iter() {
            if value.len() == 1 {
                unique_output_segments.insert(*value.iter().next().unwrap());
            }
        }
        for (_key, value) in overlaps.iter_mut() {
            if value.len() == 2 {
                let mut to_remove: Vec<char> = vec![];
                for val in value.iter() {
                    if unique_output_segments.contains(val) {
                        to_remove.push(*val);
                    }
                }
                value.remove(&to_remove[0]);
            }
        }

        // Determine the appearance of each digit with the scrambled signal wires
        let mut signal_wire_map: HashMap<char, char> = HashMap::new();
        for (segment, wires) in overlaps {
            signal_wire_map.insert(segment, *wires.iter().next().unwrap());
        }
        return signal_wire_map;
}

/// Generates the canonical appearance for all 10 digits (0-9), assuming each signal wire is
/// connected to the corresponding named segment - e.g. wire 'a' is connected to segment 'a'.
fn generate_canonical_digit_displays() -> HashMap<char, String> {
    let mut output: HashMap<char, String> = HashMap::new();
    output.insert('0', String::from("abcefg"));
    output.insert('1', String::from("cf"));
    output.insert('2', String::from("acdeg"));
    output.insert('3', String::from("acdfg"));
    output.insert('4', String::from("bcdf"));
    output.insert('5', String::from("abdfg"));
    output.insert('6', String::from("abdefg"));
    output.insert('7', String::from("acf"));
    output.insert('8', String::from("abcdefg"));
    output.insert('9', String::from("abcdfg"));
    return output;
}

/// Initialises the possible wire-segment map with empty vectors for each segment.
fn initialise_wire_possibles_map() -> HashMap<char, Vec<HashSet<char>>> {
    let mut possibles: HashMap<char, Vec<HashSet<char>>> = HashMap::new();
    possibles.insert('a', vec![]);
    possibles.insert('b', vec![]);
    possibles.insert('c', vec![]);
    possibles.insert('d', vec![]);
    possibles.insert('e', vec![]);
    possibles.insert('f', vec![]);
    possibles.insert('g', vec![]);
    return possibles;
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

    #[test]
    fn test_d08_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day8.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1011284, result);
    }
}
