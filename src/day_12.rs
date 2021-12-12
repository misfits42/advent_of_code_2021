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
        let mut current_path_visited: HashSet<String> = HashSet::new();
        current_path_visited.insert(String::from("start"));
        total_paths += visit_nodes(cave_graph, sub_cave, &mut current_path_visited);
    }
    return total_paths;
}

fn visit_nodes(
    cave_graph: &HashMap<String, HashSet<String>>,
    cave: &String,
    current_path_visited: &mut HashSet<String>,
) -> u64 {
    // We have reached the end, so a complete path has been found
    if cave == "end" {
        return 1;
    }
    // I not at the end, go to each next cave that is not a small cave already visited
    let mut total_paths = 0;
    for sub_cave in cave_graph.get(cave).unwrap() {
        // Fork the current path and visit the current cave
        let mut new_visited = current_path_visited.clone();
        new_visited.insert(cave.to_string());
        if current_path_visited.contains(sub_cave) && sub_cave.to_ascii_lowercase() == *sub_cave {
            continue;
        }
        total_paths += visit_nodes(cave_graph, sub_cave, &mut new_visited);
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
}
