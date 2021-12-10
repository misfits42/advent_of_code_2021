#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut height_map: Vec<Vec<u64>> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let map_line = line.chars().map(|x| x.to_digit(10).unwrap() as u64).collect::<Vec<u64>>();
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    // Test cases go here
    #[test]
    fn test_d09_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day9.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(558, result);
    }
}
