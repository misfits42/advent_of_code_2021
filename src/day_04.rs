use std::collections::HashSet;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<Vec<(u64, bool)>>>) {
    // Values on each board are stored in 2D array with bool to represent marking state
    let mut boards: Vec<Vec<Vec<(u64, bool)>>> = vec![];
    let mut lines = input.lines();
    // Get bingo numbers from first line
    let bingo_numbers = lines
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    // Keep reading boards until end of input reached
    loop {
        // Keep reading lines until either non-empty line or end of input reached
        let line = lines.next();
        if line.is_none() {
            break;
        } else if line.unwrap().is_empty() {
            continue;
        }
        // Read lines into a new bingo board - each tile represented by value and mark state
        let mut new_board: Vec<Vec<(u64, bool)>> = vec![];
        for i in 0..5 {
            let raw_line = {
                if i == 0 {
                    line.unwrap()
                } else {
                    lines.next().unwrap()
                }
            };
            let new_line_values = raw_line
                .trim()
                .split_ascii_whitespace()
                .map(|x| (x.parse::<u64>().unwrap(), false))
                .collect::<Vec<(u64, bool)>>();
            new_board.push(new_line_values);
        }
        boards.push(new_board);
    }
    return (bingo_numbers, boards);
}

#[aoc(day4, part1)]
fn solve_part_1(bingo_input: &(Vec<u64>, Vec<Vec<Vec<(u64, bool)>>>)) -> u64 {
    let mut boards = bingo_input.1.clone();
    // Call bingo numbers until a board wins
    for num in bingo_input.0.iter() {
        for board in boards.iter_mut() {
            // Mark the board
            let marked = mark_bingo_board(*num, board);
            // If board marked, check for win
            if marked {
                // If the board wins, calculate the final score - this is the first board to win
                if check_board_for_win(board) {
                    return calculate_board_final_score(*num, board);
                }
            }
        }
    }
    // A board should have won by now
    panic!("Day 4 Part 1 - reached end of bingo numbers without winning board!");
}

#[aoc(day4, part2)]
fn solve_part_2(bingo_input: &(Vec<u64>, Vec<Vec<Vec<(u64, bool)>>>)) -> u64 {
    let mut boards = bingo_input.1.clone();
    // Track the total number of bingo boards and the boards that have won
    let num_total_boards = boards.len();
    let mut boards_won: HashSet<usize> = HashSet::new();
    // Call bingo numbers until all boards have won
    for num in bingo_input.0.iter() {
        for i in 0..boards.len() {
            // Mark the board
            let marked = mark_bingo_board(*num, &mut boards[i]);
            // If marked, check for win
            if marked && check_board_for_win(&boards[i]) {
                boards_won.insert(i);
                // Check if the last board has finally won
                if boards_won.len() == num_total_boards {
                    return calculate_board_final_score(*num, &boards[i]);
                }
            }
        }
    }
    // Last board should have won by now
    panic!("Day 4 Part 2 - reached end of bingo numbers without all boards having won!");
}

/// Iterates over the bingo board to check if it contains the bingo number and marks it if present.
fn mark_bingo_board(bingo_number: u64, board: &mut Vec<Vec<(u64, bool)>>) -> bool {
    for y in 0..5 {
        for x in 0..5 {
            if board[y][x].0 == bingo_number {
                board[y][x].1 = true;
                return true;
            }
        }
    }
    return false;
}

/// Checks if the board has won by seeing if all numbers in a column or row have been marked.
/// Diagonals are not checked as a win condition.
fn check_board_for_win(board: &Vec<Vec<(u64, bool)>>) -> bool {
    // Check rows for win
    for y in 0..5 {
        for x in 0..5 {
            if board[y][x].1 == false {
                break;
            } else if board[y][x].1 == true && x == 4 {
                return true;
            }
        }
    }
    // Check columns for win
    for x in 0..5 {
        for y in 0..5 {
            if board[y][x].1 == false {
                break;
            } else if board[y][x].1 == true && y == 4 {
                return true;
            }
        }
    }
    // No winning condition met, so board is not a winner
    return false;
}

/// Calculates the final score of the board by summing all unmarked numbers and multiplying that
/// sum by the final bingo number called.
fn calculate_board_final_score(final_num: u64, board: &Vec<Vec<(u64, bool)>>) -> u64 {
    let mut unmarked_sum = 0;
    for y in 0..5 {
        for x in 0..5 {
            if board[y][x].1 == false {
                unmarked_sum += board[y][x].0;
            }
        }
    }
    return unmarked_sum * final_num;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d04_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day4.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(41668, result);
    }

    #[test]
    fn test_d04_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day4.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(10478, result);
    }
}
