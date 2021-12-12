use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut cave_graph: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let pair = line.split("-").collect::<Vec<&str>>();
        cave_graph
            .entry(pair[0].to_string())
            .or_insert(HashSet::new())
            .insert(pair[1].to_string());
        cave_graph
            .entry(pair[1].to_string())
            .or_insert(HashSet::new())
            .insert(pair[0].to_string());
    }
    return cave_graph;
}

#[aoc(day12, part1)]
fn solve_part_1(cave_graph: &HashMap<String, HashSet<String>>) -> u64 {
    let mut total_paths = 0;
    for sub_cave in cave_graph.get("start").unwrap() {
        let mut current_path_visited: HashMap<String, u64> = HashMap::new();
        current_path_visited.insert(String::from("start"), 1);
        total_paths += visit_nodes(cave_graph, sub_cave, &mut current_path_visited, 1, false);
    }
    return total_paths;
}

#[aoc(day12, part2)]
fn solve_part_2(cave_graph: &HashMap<String, HashSet<String>>) -> u64 {
    let mut total_paths = 0;
    for sub_cave in cave_graph.get("start").unwrap() {
        let mut current_path_visited: HashMap<String, u64> = HashMap::new();
        current_path_visited.insert(String::from("start"), 1);
        total_paths += visit_nodes(cave_graph, sub_cave, &mut current_path_visited, 2, true);
    }
    return total_paths;
}

/// Visits connected caves from the current cave, revisiting caves depending on the small cave visit
/// limit and allowance for single small cave being visited multiple times.
fn visit_nodes(
    cave_graph: &HashMap<String, HashSet<String>>,
    cave: &String,
    current_path_visited: &mut HashMap<String, u64>,
    small_visit_limit: u64,
    single_small_cave_multi: bool,
) -> u64 {
    // We have reached the end, so a complete path has been found
    if cave == "end" {
        return 1;
    }
    // If not at the end, go to each next cave that is not a small cave already visited
    let mut total_paths = 0;
    // Only track small cave visits
    if cave.to_ascii_lowercase() == *cave {
        *current_path_visited.entry(cave.to_string()).or_insert(0) += 1;
    }
    for sub_cave in cave_graph.get(cave).unwrap() {
        // Fork the current path
        let mut new_visited = current_path_visited.clone();
        // Don't revisit the start node
        if sub_cave == "start" {
            continue;
        }
        // Skip caves that cannot be revisited
        if sub_cave.to_ascii_lowercase() == *sub_cave {
            // Part 2: skip the small cave if a small cave already visited twice (visit limit 2) and
            // visiting the cave would visit it more than once
            if single_small_cave_multi
                && new_visited.contains_key(sub_cave)
                && *new_visited.values().max().unwrap() >= small_visit_limit
            {
                continue;
            // Part 1: skip the small cave if it has already been visited (visit limit 1)
            } else if !single_small_cave_multi
                && new_visited.contains_key(sub_cave)
                && *new_visited.get(sub_cave).unwrap() >= small_visit_limit
            {
                continue;
            }
        }
        total_paths += visit_nodes(
            cave_graph,
            sub_cave,
            &mut new_visited,
            small_visit_limit,
            single_small_cave_multi,
        );
    }
    return total_paths;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d12_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day12.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(3779, result);
    }

    #[test]
    fn test_d12_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day12.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(96988, result);
    }
}
