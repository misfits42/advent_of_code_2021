use std::collections::HashMap;
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
    let start_node = (0, 0);
    let end_node = (risk_map.len() - 1, risk_map[0].len() - 1);
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
    find_shortest_path_dijkstra(start_node, end_node, risk_map, &mut unvisited, &mut dists);
    // Check the resulting risk score for the bottom right tile
    return dists[end_node.1][end_node.0];
}

#[aoc(day15, part2)]
fn solve_part_2(risk_map: &Vec<Vec<u64>>) -> u64 {
    // Create new risk map
    let new_risk_map = transform_risk_map(risk_map);
    // Initialise unvisited notes and tentative distances
    let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
    let mut dists: Vec<Vec<u64>> = vec![];
    for y in 0..new_risk_map.len() {
        for x in 0..new_risk_map[y].len() {
            unvisited.insert((x, y));
        }
        dists.push(vec![u64::MAX; new_risk_map[y].len()]);
    }
    dists[0][0] = 0;
    // Shortest path
    let start_node = (0, 0);
    let end_node = (dists.len() - 1, dists[0].len() - 1);
    find_shortest_path_dijkstra(start_node, end_node, &new_risk_map, &mut unvisited, &mut dists);
    return dists[end_node.1][end_node.0];
}

/// Finds the shortest path from the start node to all other nodes reachable from it.
fn find_shortest_path_dijkstra(
    start_node: (usize, usize),
    end_node: (usize, usize),
    risk_map: &Vec<Vec<u64>>,
    unvisited: &mut HashSet<(usize, usize)>,
    dists: &mut Vec<Vec<u64>>,
) {
    let mut unvisted_dists: HashMap<(usize, usize), u64> = HashMap::new();
    unvisted_dists.insert(start_node, 0);
    while !unvisited.is_empty() {
        // Stop if there are no neighbour candidates with non-infinite distance
        if unvisted_dists.is_empty() {
            return;
        }
        // Find the unvisited node with the smallest distance
        let current_node = {
            let mut min_key: Option<(usize, usize)> = None;
            let mut min_value = u64::MAX;
            for (key, value) in unvisted_dists.iter() {
                if *value < min_value {
                    min_value = *value;
                    min_key = Some(*key);
                }
            }
            min_key.unwrap()
        };
        if current_node == end_node {
            return;
        }
        // Mark current node as visited
        unvisited.remove(&current_node);
        unvisted_dists.remove(&current_node);
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
            let existing_distance = dists[neighbour_node.1][neighbour_node.0];
            if new_distance < dists[neighbour_node.1][neighbour_node.0] {
                dists[neighbour_node.1][neighbour_node.0] = new_distance;
                unvisted_dists.insert(neighbour_node, new_distance);
            } else {
                unvisted_dists.insert(neighbour_node, existing_distance);
            }
        }
    }
}

/// Transforms the given risk map by expanding into the 5x5 tile construct for Part 2.
fn transform_risk_map(risk_map: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut new_risk_map: Vec<Vec<u64>> = vec![];
    // Get the first row of tiles input into new risk map
    for y in 0..risk_map.len() {
        let mut new_row: Vec<u64> = vec![];
        for factor in 0..5 {
            for x in 0..risk_map[y].len() {
                let new_value = {
                    let candidate = risk_map[y][x] + factor;
                    if candidate > 9 {
                        candidate - 9
                    } else {
                        candidate
                    }
                };
                new_row.push(new_value);
            }
        }
        new_risk_map.push(new_row);
    }
    // Now copy each of the rows
    for factor in 1..5 {
        for y in 0..risk_map.len() {
            let mut new_row: Vec<u64> = vec![];
            for x in 0..new_risk_map[y].len() {
                let new_value = {
                    let candidate = new_risk_map[y][x] + factor;
                    if candidate > 9 {
                        candidate - 9
                    } else {
                        candidate
                    }
                };
                new_row.push(new_value);
            }
            new_risk_map.push(new_row);
        }
    }
    return new_risk_map;
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

    #[test]
    fn test_d15_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day15.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(3012, result);
    }
}
