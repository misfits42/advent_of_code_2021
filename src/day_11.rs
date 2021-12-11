use std::collections::HashSet;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut output: Vec<Vec<u64>> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        output.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>(),
        );
    }
    return output;
}

#[aoc(day11, part1)]
fn solve_part_1(input: &Vec<Vec<u64>>) -> usize {
    let mut octopus_map = input.clone();
    let mut total_flash_count = 0;
    // Conduct 100 turns
    for _ in 0..100 {
        total_flash_count += conduct_step_octopus_map(&mut octopus_map);
    }
    return total_flash_count;
}

#[aoc(day11, part2)]
fn solve_part_2(input: &Vec<Vec<u64>>) -> usize {
    let mut octopus_map = input.clone();
    let mut turns_conducted = 0;
    loop {
        turns_conducted += 1;
        let flashes_on_turn = conduct_step_octopus_map(&mut octopus_map);
        // Check if all the octopii flashed on the current turn
        if flashes_on_turn == 100 {
            break;
        }
    }
    return turns_conducted;
}

/// Conducts one step for the current octopus map, updating it as octopii energy is changed and
/// various octopii flash.
fn conduct_step_octopus_map(octopus_map: &mut Vec<Vec<u64>>) -> usize {
    // Track which octopii have flashed on the current turn
    let mut flash_locations: HashSet<(usize, usize)> = HashSet::new();
    // Increase energy level of all octopii by 1
    let mut to_be_flashed: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..octopus_map.len() {
        for x in 0..octopus_map[y].len() {
            octopus_map[y][x] += 1;
            if octopus_map[y][x] > 9 {
                to_be_flashed.insert((x, y));
            }
        }
    }
    // Now, keep flashing octopii until all that can flash on the turn do so
    loop {
        // No more octopii waiting to flash, so go to end of turn
        if to_be_flashed.is_empty() {
            break;
        }
        // Track the octopii that are put to energy level where they will flash
        let mut next_to_flash: HashSet<(usize, usize)> = HashSet::new();
        // Process the current record of octopii waiting to flash
        for (x, y) in to_be_flashed.iter() {
            if flash_locations.contains(&(*x, *y)) {
                continue;
            }
            flash_locations.insert((*x, *y));
            // Increase energy level of all surrounding points by 1
            let neighbour_points =
                get_surrounding_points(*x, *y, octopus_map[*y].len() - 1, octopus_map.len() - 1);
            for (x_nb, y_nb) in neighbour_points {
                // Skip if already flashed this turn
                if flash_locations.contains(&(x_nb, y_nb)) {
                    continue;
                }
                octopus_map[y_nb][x_nb] += 1;
                if octopus_map[y_nb][x_nb] > 9 {
                    next_to_flash.insert((x_nb, y_nb));
                }
            }
        }
        // Set all flashed octopii to energy 0
        for (x, y) in to_be_flashed {
            octopus_map[y][x] = 0;
        }
        to_be_flashed = next_to_flash;
    }
    return flash_locations.len();
}

/// Calculates the points around (x, y) in a 2D grid with non-negative indices bounded by given
/// maximum x- and y-values.
fn get_surrounding_points(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    // Top left
    if x == 0 && y == 0 {
        vec![(x, y + 1), (x + 1, y + 1), (x + 1, y)]
    // Bottom left
    } else if x == 0 && y == y_max {
        vec![(x, y - 1), (x + 1, y - 1), (x + 1, y)]
    // Mid left
    } else if x == 0 && y < y_max {
        vec![
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
        ]
    // Top right
    } else if x == x_max && y == 0 {
        vec![(x - 1, y), (x - 1, y + 1), (x, y + 1)]
    // Top mid
    } else if x < x_max && y == 0 {
        vec![
            (x - 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
            (x + 1, y),
        ]
    // Bottom right
    } else if x == x_max && y == y_max {
        vec![(x - 1, y), (x - 1, y - 1), (x, y - 1)]
    // Bottom mid
    } else if x < x_max && y == y_max {
        vec![
            (x - 1, y),
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
        ]
    // Mid right
    } else if x == x_max && y < y_max {
        vec![
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
            (x, y - 1),
        ]
    } else {
        vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d11_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day11.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(1617, result);
    }

    #[test]
    fn test_d11_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day11.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(258, result);
    }
}
