use std::collections::HashSet;

use regex::Regex;

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Vec<(bool, i64, i64, i64, i64, i64, i64)> {
    let mut reboot_commands: Vec<(bool, i64, i64, i64, i64, i64, i64)> = vec![];
    let line_regex =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let captures = line_regex.captures(line).unwrap();
        let on_state = {
            if &captures[1] == "on" {
                true
            } else {
                false
            }
        };
        let x_min = captures[2].parse::<i64>().unwrap();
        let x_max = captures[3].parse::<i64>().unwrap();
        let y_min = captures[4].parse::<i64>().unwrap();
        let y_max = captures[5].parse::<i64>().unwrap();
        let z_min = captures[6].parse::<i64>().unwrap();
        let z_max = captures[7].parse::<i64>().unwrap();
        let cmd = (on_state, x_min, x_max, y_min, y_max, z_min, z_max);
        reboot_commands.push(cmd);
    }
    return reboot_commands;
}

#[aoc(day22, part1)]
fn solve_part_1(reboot_commands: &Vec<(bool, i64, i64, i64, i64, i64, i64)>) -> usize {
    let mut reactor_cubes_on: HashSet<(i64, i64, i64)> = HashSet::new();
    // println!(">>>> {:?}", reboot_commands);
    let commands_filtered: Vec<(bool, i64, i64, i64, i64, i64, i64)> = reboot_commands
        .iter()
        .map(|x| *x)
        .filter(|v| v.1 >= -50 && v.2 <= 50 && v.3 >= -50 && v.4 <= 50 && v.5 >= -50 && v.6 <= 50)
        .collect();
    for (on_state, x_min, x_max, y_min, y_max, z_min, z_max) in commands_filtered {
        for z in z_min..=z_max {
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    if on_state {
                        reactor_cubes_on.insert((x, y, z));
                    } else {
                        reactor_cubes_on.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    return reactor_cubes_on.len();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d22_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day22.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(615700, result);
    }
}
