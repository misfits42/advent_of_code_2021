#[aoc_generator(day16)]
fn parse_input(input: &str) -> String {
    let mut binary_string = String::new();
    let data = input.lines().next().unwrap().trim().to_ascii_lowercase();
    for c in data.chars() {
        let binary_decode = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'a' => "1010",
            'b' => "1011",
            'c' => "1100",
            'd' => "1101",
            'e' => "1110",
            'f' => "1111",
            _ => panic!("Day 16 - invalid character in input: {}", c),
        };
        binary_string.push_str(binary_decode);
    }
    return binary_string;
}

#[aoc(day16, part1)]
fn solve_part_1(binary_string: &String) -> u64 {
    let mut version_sum = 0;
    let binary_digits = binary_string.chars().collect::<Vec<char>>();
    let mut cursor = 0;
    // Handle the single top-level packet
    // Check version for current packet
    let version_id = process_version_or_type_number(&binary_digits, &mut cursor);
    version_sum += version_id;
    // Check packet type
    let type_id = process_version_or_type_number(&binary_digits, &mut cursor);
    // Handle literal packet
    if type_id == 4 {
        let (_literal, _total_packet_chars) =
            process_literal_packet(&binary_digits, &mut cursor, true);
    // Handle operator packet
    } else {
        let (_result, sub_version_sum, _total_packet_chars) =
            process_operator_packet(&binary_digits, &mut cursor, type_id, true);
        version_sum += sub_version_sum;
    }
    return version_sum;
}

#[aoc(day16, part2)]
fn solve_part_2(binary_string: &String) -> u64 {
    let binary_digits = binary_string.chars().collect::<Vec<char>>();
    let mut cursor = 0;
    // Handle the single top level packet
    let _version_id = process_version_or_type_number(&binary_digits, &mut cursor);
    let type_id = process_version_or_type_number(&binary_digits, &mut cursor);
    let result = {
        if type_id == 4 {
            let (literal, _total_packet_chars) =
                process_literal_packet(&binary_digits, &mut cursor, true);
            literal
        } else {
            let (result, _sub_version_sum, _total_packet_chars) =
                process_operator_packet(&binary_digits, &mut cursor, type_id, true);
            result
        }
    };
    return result;
}

fn process_operator_packet(
    binary_digits: &Vec<char>,
    cursor: &mut usize,
    op_type: u64,
    has_zero_padding: bool,
) -> (u64, u64, usize) {
    let mut version_sum = 0;
    // Take into account the packet version and type already processed
    let mut total_packet_chars = 6;
    // Determine operator packet type (0 or 1)
    let operator_mode = binary_digits[*cursor].to_digit(10).unwrap();
    *cursor += 1;
    total_packet_chars += 1;
    // Record values from nested packets
    let mut sub_results: Vec<u64> = vec![];
    // Calculate L value
    let l_val = {
        let mut l_string = String::new();
        let len = {
            if operator_mode == 0 {
                15
            } else {
                11
            }
        };
        for i in 0..len {
            l_string.push(binary_digits[*cursor + i]);
        }
        *cursor += len;
        total_packet_chars += len;
        usize::from_str_radix(&l_string, 2).unwrap()
    };
    // Process remainder of operator packet
    if operator_mode == 0 {
        let mut total_subpacket_chars = 0;
        while total_subpacket_chars < l_val {
            let version_id = process_version_or_type_number(binary_digits, cursor);
            version_sum += version_id;
            let type_id = process_version_or_type_number(binary_digits, cursor);
            if type_id == 4 {
                let (literal, total_chars) = process_literal_packet(binary_digits, cursor, false);
                sub_results.push(literal);
                total_subpacket_chars += total_chars;
            } else {
                let (result, sub_version_sum, total_chars) =
                    process_operator_packet(binary_digits, cursor, type_id, false);
                sub_results.push(result);
                version_sum += sub_version_sum;
                total_subpacket_chars += total_chars;
            }
        }
        total_packet_chars += total_subpacket_chars;
    } else {
        let mut total_subpacket_chars = 0;
        for _ in 0..l_val {
            let version_id = process_version_or_type_number(binary_digits, cursor);
            version_sum += version_id;
            let type_id = process_version_or_type_number(binary_digits, cursor);
            if type_id == 4 {
                let (literal, total_chars) = process_literal_packet(binary_digits, cursor, false);
                sub_results.push(literal);
                total_subpacket_chars += total_chars;
            } else {
                let (result, sub_version_sum, total_chars) =
                    process_operator_packet(binary_digits, cursor, type_id, false);
                sub_results.push(result);
                version_sum += sub_version_sum;
                total_subpacket_chars += total_chars;
            }
        }
        total_packet_chars += total_subpacket_chars;
    }
    // Adjust for zero padding
    if has_zero_padding {
        *cursor += total_packet_chars % 8;
    }
    // Calculate the result for current operator packet
    let result = match op_type {
        0 => sub_results.iter().sum::<u64>(),
        1 => sub_results.iter().product::<u64>(),
        2 => *sub_results.iter().min().unwrap(),
        3 => *sub_results.iter().max().unwrap(),
        5 => {
            if sub_results[0] > sub_results[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if sub_results[0] < sub_results[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if sub_results[0] == sub_results[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("Day 16 - bad operator packet type ID: {}", op_type),
    };
    return (result, version_sum, total_packet_chars);
}

/// Processes a literal value
fn process_literal_packet(
    binary_digits: &Vec<char>,
    cursor: &mut usize,
    has_zero_padding: bool,
) -> (u64, usize) {
    let mut literal_string = String::new();
    let mut final_group_observed = false;
    // Take into account the packet version and type already processed
    let mut total_packet_chars = 6;
    while !final_group_observed {
        // Check for final group in packet
        if binary_digits[*cursor] == '0' {
            final_group_observed = true;
        }
        for i in 1..=4 {
            literal_string.push(binary_digits[*cursor + i]);
        }
        *cursor += 5;
        total_packet_chars += 5;
    }
    // Adjust cursor to account for zero-padding
    if has_zero_padding {
        *cursor += total_packet_chars % 8;
    }
    return (
        u64::from_str_radix(&literal_string, 2).unwrap(),
        total_packet_chars,
    );
}

/// Takes the three characters starting from the cursor and converts them into an integer value.
/// Cursor is advanced three places.
fn process_version_or_type_number(binary_digits: &Vec<char>, cursor: &mut usize) -> u64 {
    let mut digits = String::new();
    for i in 0..3 {
        digits.push(binary_digits[*cursor + i]);
    }
    *cursor += 3;
    return u64::from_str_radix(&digits, 2).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d16_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day16.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(947, result);
    }

    #[test]
    fn test_d16_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day16.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(660797830937, result);
    }

    #[test]
    fn test_d16_p1_test_001() {
        let input = parse_input(&read_to_string("./input/2021/test/day_16_test_001.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(6, result);
    }

    #[test]
    fn test_d16_p1_test_002() {
        let input = parse_input(&read_to_string("./input/2021/test/day_16_test_002.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(9, result);
    }

    #[test]
    fn test_d16_p1_test_003() {
        let input = parse_input(&read_to_string("./input/2021/test/day_16_test_003.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(14, result);
    }

    #[test]
    fn test_d16_p1_test_004() {
        let input = parse_input(&read_to_string("./input/2021/test/day_16_test_004.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(16, result);
    }

    #[test]
    fn test_d16_p1_test_005() {
        let input = parse_input(&read_to_string("./input/2021/test/day_16_test_005.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(12, result);
    }

    #[test]
    fn test_d16_p1_test_006() {
        let input = parse_input(&read_to_string("./input/2021/test/day_16_test_006.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(23, result);
    }

    #[test]
    fn test_d16_p1_test_007() {
        let input = parse_input(&read_to_string("./input/2021/test/day_16_test_007.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(31, result);
    }
}
