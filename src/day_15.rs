use std::collections::HashSet;

use super::utils::map::*;

#[aoc_generator(day15)]
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

#[aoc(day15, part1)]
fn solve_part_1(risk_map: &Vec<Vec<u64>>) -> u64 {
    // Initialise unvisited notes and tentative distances
    let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
    let mut dists: Vec<Vec<u64>> = vec![];
    for y in 0..risk_map.len() {
        for x in 0..risk_map[y].len() {
            unvisited.insert((x, y));
        }
        dists.push(vec![u64::MAX; risk_map[y].len()]);
    }
    dists[0][0] = 0;
    // Find the path
    while !unvisited.is_empty() {
        // Find the unvisited node with the smallest distance
        let current_node = {
            let mut min_distance = u64::MAX;
            let mut candidate_node: Option<(usize, usize)> = None;
            for y in 0..dists.len() {
                for x in 0..dists[y].len() {
                    if unvisited.contains(&(x, y)) && dists[y][x] < min_distance {
                        min_distance = dists[y][x];
                        candidate_node = Some((x, y));
                    }
                }
            }
            candidate_node.unwrap()
        };
        // Mark current node as visited
        unvisited.remove(&current_node);
        // Determine distance from current node for all unvisited neighbour nodes
        let neighbour_nodes = get_surrounding_points_no_diagonals(
            current_node.0,
            current_node.1,
            dists[0].len() - 1,
            dists.len() - 1,
        );
        for neighbour_node in neighbour_nodes {
            if !unvisited.contains(&neighbour_node) {
                continue;
            }
            let new_distance = dists[current_node.1][current_node.0]
                + risk_map[neighbour_node.1][neighbour_node.0];
            if new_distance < dists[neighbour_node.1][neighbour_node.0] {
                dists[neighbour_node.1][neighbour_node.0] = new_distance;
            }
        }
    }
    // Check the resulting risk score for the bottom right tile
    return dists[dists.len() - 1][dists[0].len() - 1];
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d15_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day15.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(811, result);
    }
}
