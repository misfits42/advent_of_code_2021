use std::collections::VecDeque;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        output.push(line.to_string());
    }
    return output;
}

#[aoc(day10, part1)]
fn solve_part_1(navsys_lines: &Vec<String>) -> u64 {
    let mut total_syntax_error_score = 0;
    for line in navsys_lines {
        let (syntax_error_score, _brace_stack) = check_for_corrupted_line(line);
        total_syntax_error_score += syntax_error_score;
    }
    return total_syntax_error_score;
}

#[aoc(day10, part2)]
fn solve_part_2(navsys_lines: &Vec<String>) -> u64 {
    // First filter out the corrupted and valid lines - so only incomplete lines remain
    let mut incomplete_brace_stacks: Vec<VecDeque<char>> = vec![];
    let mut autocomplete_scores: Vec<u64> = vec![];
    for line in navsys_lines {
        let (syntax_error_score, brace_stack) = check_for_corrupted_line(line);
        if syntax_error_score == 0 && !brace_stack.is_empty() {
            incomplete_brace_stacks.push(brace_stack.clone());
        }
    }
    // Now calculate the autocomplete scores for all the incomplete lines
    for brace_stack in incomplete_brace_stacks {
        let mut completion_string = String::new();
        // Iterate over brace stack from back to generate the completion string
        for brace in brace_stack.iter().rev() {
            let closing_brace = {
                match brace {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => panic!("Day 10 Part 2 - invalid character in brace stack: {}", brace)
                }
            };
            completion_string.push(closing_brace);
        }
        // Calculate the autocomplete score
        let mut score = 0;
        for brace in completion_string.chars() {
            score *= 5;
            match brace {
                ')' => score += 1,
                ']' => score += 2,
                '}' => score += 3,
                '>' => score += 4,
                _ => panic!("Day 10 Part 2 - invalid character in completion string: {}", brace)
            }
        }
        autocomplete_scores.push(score);
    }
    // Sort the autocomplete scores and pick the middle score (assuming odd number of scores)
    autocomplete_scores.sort();
    let i_mid = (autocomplete_scores.len() - 1) / 2;
    return autocomplete_scores[i_mid]
}

/// Checks if the given line is corrupted, i.e. a closing brace appears in an invalid location.
///
/// - Return value part 0: value is 0 for valid and incomplete lines, and greater than 0 for corrupted lines (being
///   the syntax error score).
/// - Return value part 1: state of brace stack when the check is completed.
fn check_for_corrupted_line(navsys_line: &String) -> (u64, VecDeque<char>) {
    // Use a stack to record opening brace
    let mut brace_stack: VecDeque<char> = VecDeque::new();
    for c in navsys_line.chars() {
        match c {
            // Add the opening braces to the stack
            '(' | '[' | '{' | '<' => {
                brace_stack.push_back(c);
            }
            // For closing braces, check if there are any open braces and if last one is closed
            ')' => {
                if brace_stack.is_empty() || brace_stack.pop_back().unwrap() != '(' {
                    return (3, brace_stack);
                }
            }
            ']' => {
                if brace_stack.is_empty() || brace_stack.pop_back().unwrap() != '[' {
                    return (57, brace_stack);
                }
            }
            '}' => {
                if brace_stack.is_empty() || brace_stack.pop_back().unwrap() != '{' {
                    return (1197, brace_stack);
                }
            }
            '>' => {
                if brace_stack.is_empty() || brace_stack.pop_back().unwrap() != '<' {
                    return (25137, brace_stack);
                }
            }
            _ => {
                panic!("Day 1 Part 1 - illegal character in Navsys chunk: {}", c);
            }
        }
    }
    // Given line is either valid or incomplete, so gets a syntax error score of 0.
    return (0, brace_stack);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d10_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day10.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(464991, result);
    }

    #[test]
    fn test_d10_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day10.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(3662008566, result);
    }
}
