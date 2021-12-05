use std::collections::HashSet;

use regex::Regex;

use super::utils::map::Point2D;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<(Point2D, Point2D)> {
    let mut output: Vec<(Point2D, Point2D)> = vec![];
    let line_regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    for line in input.lines() {
        // Ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let captures = line_regex.captures(line).unwrap();
        let x1 = captures[1].parse::<i64>().unwrap();
        let y1 = captures[2].parse::<i64>().unwrap();
        let x2 = captures[3].parse::<i64>().unwrap();
        let y2 = captures[4].parse::<i64>().unwrap();
        let point_1 = Point2D::new(x1, y1);
        let point_2 = Point2D::new(x2, y2);
        output.push((point_1, point_2));
    }
    return output;
}

#[aoc(day5, part1)]
fn solve_part_1(vent_lines: &Vec<(Point2D, Point2D)>) -> usize {
    let mut observed_points: HashSet<Point2D> = HashSet::new();
    let mut overlap_points: HashSet<Point2D> = HashSet::new();
    // Only consider vertical or horizontal lines - diagonals are ignored for this part!
    for (point_1, point_2) in vent_lines {
        // Ignore diagonals
        if point_1.get_x() != point_2.get_x() && point_1.get_y() != point_2.get_y() {
            continue;
        }
        mark_points_on_map(point_1, point_2, &mut observed_points, &mut overlap_points);
    }
    return overlap_points.len();
}

#[aoc(day5, part2)]
fn solve_part_2(vent_lines: &Vec<(Point2D, Point2D)>) -> usize {
    let mut observed_points: HashSet<Point2D> = HashSet::new();
    let mut overlap_points: HashSet<Point2D> = HashSet::new();
    // Consider all lines, including diagonals (assumed 45-degree slope only)
    for (point_1, point_2) in vent_lines {
        mark_points_on_map(point_1, point_2, &mut observed_points, &mut overlap_points);
    }
    return overlap_points.len();
}

/// Marks all points on the map between the specified start and end points. All observed and
/// overlapping points are recorded in the given HashSets.
fn mark_points_on_map(
    point_1: &Point2D,
    point_2: &Point2D,
    observed: &mut HashSet<Point2D>,
    overlap: &mut HashSet<Point2D>,
) {
    // Calculate the co-ordinate deltas so we know what direction the line is pointing
    let delta_x = (point_2.get_x() - point_1.get_x()).signum();
    let delta_y = (point_2.get_y() - point_1.get_y()).signum();
    let mut point = point_1.clone();
    // Keep adding points until we get to the end point
    while point.get_x() != point_2.get_x() + delta_x || point.get_y() != point_2.get_y() + delta_y {
        let is_new_point = observed.insert(point);
        if !is_new_point {
            overlap.insert(point);
        }
        point.move_point(delta_x, delta_y);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d05_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day5.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(6007, result);
    }

    #[test]
    fn test_d05_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day5.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(19349, result);
    }
}
