/// Represents the 100-sided deterministic die used in AOC 2021 Day 21.
struct DeterministicDie {
    times_rolled: u64,
    next_roll: u64
}

impl DeterministicDie {
    pub fn new() -> Self {
        Self {
            times_rolled: 0,
            next_roll: 1,
        }
    }

    pub fn roll(&mut self) -> u64 {
        self.times_rolled += 1;
        let this_roll = self.next_roll;
        self.next_roll += 1;
        if self.next_roll > 100 {
            self.next_roll = 1;
        }
        return this_roll;
    }
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> (u64, u64) {
    let mut input_lines = input.lines();
    let p1_start = input_lines.next().unwrap().trim().split(": ").collect::<Vec<&str>>()[1]
        .parse::<u64>().unwrap();
    let p2_start = input_lines.next().unwrap().trim().split(": ").collect::<Vec<&str>>()[1]
        .parse::<u64>().unwrap();
    return (p1_start, p2_start);
}

#[aoc(day21, part1)]
fn solve_part_1(player_start_pos: &(u64, u64)) -> u64 {
    let (mut p1_pos, mut p2_pos) = player_start_pos;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut det_die = DeterministicDie::new();
    loop {
        // Player 1 turn
        let mut p1_rolls = 0;
        for _ in 0..3 {
            p1_rolls += det_die.roll();
        }
        p1_pos += p1_rolls % 10;
        if p1_pos > 10 {
            p1_pos = p1_pos - 10;
        }
        p1_score += p1_pos;
        if p1_score >= 1000 {
            break;
        }
        // Player 2 turn
        let mut p2_rolls = 0;
        for _ in 0..3 {
            p2_rolls += det_die.roll();
        }
        p2_pos += p2_rolls % 10;
        if p2_pos > 10 {
            p2_pos = p2_pos - 10;
        }
        p2_score += p2_pos;
        if p2_score >= 1000 {
            break;
        }
    }
    if p1_score < p2_score {
        return p1_score * det_die.times_rolled;
    } else {
        return p2_score * det_die.times_rolled;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d21_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day21.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(920079, result);
    }
}
