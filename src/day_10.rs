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
        // Use a stack to record opening brace
        let mut brace_stack: VecDeque<char> = VecDeque::new();
        for c in line.chars() {
            match c {
                // Add the opening braces to the stack
                '(' | '[' | '{' | '<' => {
                    brace_stack.push_back(c);
                },
                // For closing braces, check if there are any open braces and if last one is closed
                ')' => {
                    if brace_stack.is_empty() || brace_stack.pop_back().unwrap() != '(' {
                        total_syntax_error_score += 3;
                        break;
                    }
                },
                ']' => {
                    if brace_stack.is_empty() || brace_stack.pop_back().unwrap() != '[' {
                        total_syntax_error_score += 57;
                        break;
                    }
                },
                '}' => {
                    if brace_stack.is_empty() || brace_stack.pop_back().unwrap() != '{' {
                        total_syntax_error_score += 1197;
                        break;
                    }
                },
                '>' => {
                    if brace_stack.is_empty() || brace_stack.pop_back().unwrap() != '<' {
                        total_syntax_error_score += 25137;
                        break;
                    }
                },
                _ => {
                    panic!("Day 1 Part 1 - illegal character in Navsys chunk: {}", c);
                }
            }
        }
        // For Part 1 - we aren't looking for incomplete lines, so doesn't matter if brace stack
        // still has braces sitting in it at end of the line

    }
    return total_syntax_error_score;
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
}
