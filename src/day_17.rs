use std::collections::HashSet;

use regex::Regex;

struct TargetArea {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl TargetArea {
    pub fn new(x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> Self {
        Self {
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max,
        }
    }
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> TargetArea {
    let input_regex = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    let input_line = input.lines().next().unwrap().trim();
    let captures = input_regex.captures(input_line).unwrap();
    let x_min = captures[1].parse::<i64>().unwrap();
    let x_max = captures[2].parse::<i64>().unwrap();
    let y_min = captures[3].parse::<i64>().unwrap();
    let y_max = captures[4].parse::<i64>().unwrap();
    return TargetArea::new(x_min, x_max, y_min, y_max);
}

#[aoc(day17, part1)]
fn solve_part_1(target_area: &TargetArea) -> i64 {
    let mut max_y_position = i64::MIN;
    for y in -500..500 {
        for x in -500..500 {
            let mut pos = (0, 0);
            let mut vel = (x, y);
            let mut reached_target_area = false;
            let mut current_max_y_pos = i64::MIN;
            loop {
                if pos.1 > current_max_y_pos {
                    current_max_y_pos = pos.1;
                }
                // Check if initial conditions mean the probe misses the target area
                if pos.0 > target_area.x_max || pos.1 < target_area.y_min {
                    break;
                }
                // Update probe position and velocity
                pos.0 += vel.0;
                pos.1 += vel.1;
                if vel.0 < 0 {
                    vel.0 += 1;
                } else if vel.0 > 0 {
                    vel.0 -= 1;
                }
                vel.1 -= 1;
                // Check if probe is in the target area
                if pos.0 >= target_area.x_min
                    && pos.0 <= target_area.x_max
                    && pos.1 >= target_area.y_min
                    && pos.1 <= target_area.y_max
                {
                    reached_target_area = true;
                    break;
                }
            }
            if reached_target_area && current_max_y_pos > max_y_position {
                max_y_position = current_max_y_pos;
            }
        }
    }
    return max_y_position;
}

#[aoc(day17, part2)]
fn solve_part_2(target_area: &TargetArea) -> usize {
    let mut good_vels: HashSet::<(i64, i64)> = HashSet::new();
    for y in -500..500 {
        for x in -500..500 {
            let mut pos = (0, 0);
            let mut vel = (x, y);
            let mut reached_target_area = false;
            loop {
                // Check if initial conditions mean the probe misses the target area
                if pos.0 > target_area.x_max || pos.1 < target_area.y_min {
                    break;
                }
                // Update probe position and velocity
                pos.0 += vel.0;
                pos.1 += vel.1;
                if vel.0 < 0 {
                    vel.0 += 1;
                } else if vel.0 > 0 {
                    vel.0 -= 1;
                }
                vel.1 -= 1;
                // Check if probe is in the target area
                if pos.0 >= target_area.x_min
                    && pos.0 <= target_area.x_max
                    && pos.1 >= target_area.y_min
                    && pos.1 <= target_area.y_max
                {
                    reached_target_area = true;
                    break;
                }
            }
            if reached_target_area {
                good_vels.insert((x, y));
            }
        }
    }
    return good_vels.len();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d17_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day17.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(7875, result);
    }

    #[test]
    fn test_d17_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day17.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(2321, result);
    }
}
