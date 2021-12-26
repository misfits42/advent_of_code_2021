/// Represents the 100-sided deterministic die used in AOC 2021 Day 21.
struct DeterministicDie {
    times_rolled: u64,
    next_roll: u64,
}

impl DeterministicDie {
    /// Creates a new DeterministicDie with starting roll of 1.
    pub fn new() -> Self {
        Self {
            times_rolled: 0,
            next_roll: 1,
        }
    }

    /// Rolls the die and rotates the next roll value.
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

/// Represents the state of universe required to play successive turns in the Dirac Dice game.
#[derive(Copy, Clone)]
struct UniverseState {
    p1_pos: u64,
    p2_pos: u64,
    p1_score: u64,
    p2_score: u64,
    universe_mult: u64,
}

impl UniverseState {
    pub fn new(p1_pos: u64, p2_pos: u64) -> Self {
        Self {
            p1_pos: p1_pos,
            p2_pos: p2_pos,
            p1_score: 0,
            p2_score: 0,
            universe_mult: 1,
        }
    }
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> (u64, u64) {
    let mut input_lines = input.lines();
    let p1_start = input_lines
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .parse::<u64>()
        .unwrap();
    let p2_start = input_lines
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .parse::<u64>()
        .unwrap();
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

#[aoc(day21, part2)]
fn solve_part_2(player_start_pos: &(u64, u64)) -> u64 {
    let universe_state = UniverseState::new(player_start_pos.0, player_start_pos.1);
    // Generate possible roll values and counts for possible combos of three Dirac dice rolls
    let roll_val_counts: Vec<(u64, u64)> =
        vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let (p1_wins, p2_wins) = play_turn_dirac(&universe_state, &roll_val_counts);
    if p1_wins > p2_wins {
        return p1_wins;
    } else {
        return p2_wins;
    }
}

/// Plays a turn of the Dirac dice game, with input universe split as required after Player 1 and
/// Player 2 turns.
fn play_turn_dirac(universe_0: &UniverseState, roll_val_counts: &Vec<(u64, u64)>) -> (u64, u64) {
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    // P1 turns
    let mut candidate_universes: Vec<UniverseState> = vec![];
    for (roll_val, count) in roll_val_counts {
        let mut universe_state_split = universe_0.clone();
        universe_state_split.p1_pos += roll_val;
        universe_state_split.universe_mult *= count;
        if universe_state_split.p1_pos > 10 {
            universe_state_split.p1_pos = universe_state_split.p1_pos - 10;
        }
        universe_state_split.p1_score += universe_state_split.p1_pos;
        if universe_state_split.p1_score >= 21 {
            p1_wins += universe_state_split.universe_mult;
        } else {
            candidate_universes.push(universe_state_split);
        }
    }
    // P2 turns
    let mut new_candidate_universes: Vec<UniverseState> = vec![];
    for universe in candidate_universes {
        for (roll_val, count) in roll_val_counts {
            let mut universe_state_split = universe.clone();
            universe_state_split.p2_pos += roll_val;
            universe_state_split.universe_mult *= count;
            if universe_state_split.p2_pos > 10 {
                universe_state_split.p2_pos = universe_state_split.p2_pos - 10;
            }
            universe_state_split.p2_score += universe_state_split.p2_pos;
            if universe_state_split.p2_score >= 21 {
                p2_wins += universe_state_split.universe_mult;
            } else {
                new_candidate_universes.push(universe_state_split);
            }
        }
    }
    // Keep playing for all universes without a winner yet
    for universe in new_candidate_universes {
        let (sub_p1_wins, sub_p2_wins) = play_turn_dirac(&universe, roll_val_counts);
        p1_wins += sub_p1_wins;
        p2_wins += sub_p2_wins;
    }
    return (p1_wins, p2_wins);
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

    #[test]
    fn test_d21_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day21.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(56852759190649, result);
    }
}
