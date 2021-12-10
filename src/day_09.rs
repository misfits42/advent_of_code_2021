use std::collections::HashSet;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut height_map: Vec<Vec<u64>> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let map_line = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        height_map.push(map_line);
    }
    return height_map;
}

#[aoc(day9, part1)]
fn solve_part_1(height_map: &Vec<Vec<u64>>) -> u64 {
    let mut total_risk_score = 0;
    for y in 0..height_map.len() {
        for x in 0..height_map[y].len() {
            // Determine heights around the current tile
            let mut surrounding_heights: Vec<u64> = vec![];
            if y > 0 {
                surrounding_heights.push(height_map[y - 1][x]);
            }
            if y < height_map.len() - 1 {
                surrounding_heights.push(height_map[y + 1][x]);
            }
            if x > 0 {
                surrounding_heights.push(height_map[y][x - 1]);
            }
            if x < height_map[y].len() - 1 {
                surrounding_heights.push(height_map[y][x + 1]);
            }
            surrounding_heights.sort();
            // Check if current tile is a low point
            if height_map[y][x] < surrounding_heights[0] {
                total_risk_score += height_map[y][x] + 1;
            }
        }
    }
    return total_risk_score;
}

#[aoc(day9, part2)]
fn solve_part_2(height_map: &Vec<Vec<u64>>) -> usize {
    // Record all the basins discovered and all points so far included in a basin
    let mut basins: Vec<HashSet<(usize, usize)>> = vec![];
    let mut observed: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..height_map.len() {
        for x in 0..height_map[y].len() {
            // Check we are not on a peak
            if height_map[y][x] == 9 {
                observed.insert((x, y));
                continue;
            }
            // Check if we have a new basin
            if !observed.contains(&(x, y)) {
                let mut basin: HashSet<(usize, usize)> = HashSet::new();
                add_to_basin_recursive(x, y, height_map, &mut basin, &mut observed);
                basins.push(basin);
            }
        }
    }
    // Calculate the product of the size of the three largest basins
    let mut basin_sizes = basins.iter().map(|x| x.len()).collect::<Vec<usize>>();
    basin_sizes.sort();
    let mut product = 1;
    for i in (basin_sizes.len() - 3)..=(basin_sizes.len() - 1) {
        product *= basin_sizes[i];
    }
    return product;
}

/// Adds points to the current basin using breadth-first recursion.
fn add_to_basin_recursive(
    x: usize,
    y: usize,
    height_map: &Vec<Vec<u64>>,
    basin: &mut HashSet<(usize, usize)>,
    observed: &mut HashSet<(usize, usize)>,
) {
    // Add current point to the basin
    basin.insert((x, y));
    observed.insert((x, y));
    // Check if the surrounding points are part of the basin
    if y > 0 && height_map[y - 1][x] < 9 && !observed.contains(&(x, y - 1)) {
        add_to_basin_recursive(x, y - 1, height_map, basin, observed);
    }
    if y < height_map.len() - 1 && height_map[y + 1][x] < 9 && !observed.contains(&(x, y + 1)) {
        add_to_basin_recursive(x, y + 1, height_map, basin, observed);
    }
    if x > 0 && height_map[y][x - 1] < 9 && !observed.contains(&(x - 1, y)) {
        add_to_basin_recursive(x - 1, y, height_map, basin, observed);
    }
    if x < height_map[y].len() - 1 && height_map[y][x + 1] < 9 && !observed.contains(&(x + 1, y)) {
        add_to_basin_recursive(x + 1, y, height_map, basin, observed);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d09_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day9.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(558, result);
    }

    #[test]
    fn test_d09_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day9.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(882942, result);
    }
}
