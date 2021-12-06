#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct LanternFish {
    timer: u64
}

impl LanternFish {
    pub fn new(initial_age: u64) -> Self {
        Self {
            timer: initial_age
        }
    }

    pub fn decrement_timer(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            return true;
        } else {
            self.timer -= 1;
            return false;
        }
    }
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<LanternFish> {
    let mut output: Vec<LanternFish> = vec![];
    let initial_ages = input.lines().next().unwrap().split(",").map(|x| x.parse::<u64>().unwrap());
    for age in initial_ages {
        let lantern_fish = LanternFish::new(age);
        output.push(lantern_fish);
    }
    return output;
}

#[aoc(day6, part1)]
fn solve_part_1(initial_fish: &Vec<LanternFish>) -> usize {
    let mut fish_pop = initial_fish.clone();
    // Simulate for 80 days
    for _ in 0..80 {
        // For each day, track how many new fish are made
        let mut new_fish_spawned = 0;
        for fish in fish_pop.iter_mut() {
            let new_fish = fish.decrement_timer();
            if new_fish {
                new_fish_spawned += 1;
            }
        }
        for _ in 0..new_fish_spawned {
            fish_pop.push(LanternFish::new(8));
        }
    }
    return fish_pop.len();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    // Test cases go here
    #[test]
    fn test_d06_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day6.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(366057, result);
    }
}
