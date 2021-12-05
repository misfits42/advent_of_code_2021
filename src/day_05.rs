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
        // Check for vertical line
        if point_1.get_x() == point_2.get_x() {
            let y_vals: Vec<i64> = vec![point_1.get_y(), point_2.get_y()];
            let y_min = *y_vals.iter().min().unwrap();
            let y_max = *y_vals.iter().max().unwrap();
            for y in y_min..=y_max {
                let point = Point2D::new(point_1.get_x(), y);
                let is_new_point = observed_points.insert(point);
                if !is_new_point {
                    overlap_points.insert(point);
                }
            }
        // Check for horizontal line
        } else if point_1.get_y() == point_2.get_y() {
            let x_vals: Vec<i64> = vec![point_1.get_x(), point_2.get_x()];
            let x_min = *x_vals.iter().min().unwrap();
            let x_max = *x_vals.iter().max().unwrap();
            for x in x_min..=x_max {
                let point = Point2D::new(x, point_1.get_y());
                let is_new_point = observed_points.insert(point);
                if !is_new_point {
                    overlap_points.insert(point);
                }
            }
        }
    }
    return overlap_points.len();
}

#[aoc(day5, part2)]
fn solve_part_2(vent_lines: &Vec<(Point2D, Point2D)>) -> usize {
    let mut observed_points: HashSet<Point2D> = HashSet::new();
    let mut overlap_points: HashSet<Point2D> = HashSet::new();
    // Consider all lines, including diagonals (assumed 45-degree slope only)
    for (point_1, point_2) in vent_lines {
        // Check for vertical line
        if point_1.get_x() == point_2.get_x() {
            let y_vals: Vec<i64> = vec![point_1.get_y(), point_2.get_y()];
            let y_min = *y_vals.iter().min().unwrap();
            let y_max = *y_vals.iter().max().unwrap();
            for y in y_min..=y_max {
                let point = Point2D::new(point_1.get_x(), y);
                let is_new_point = observed_points.insert(point);
                if !is_new_point {
                    overlap_points.insert(point);
                }
            }
        // Check for horizontal line
        } else if point_1.get_y() == point_2.get_y() {
            let x_vals: Vec<i64> = vec![point_1.get_x(), point_2.get_x()];
            let x_min = *x_vals.iter().min().unwrap();
            let x_max = *x_vals.iter().max().unwrap();
            for x in x_min..=x_max {
                let point = Point2D::new(x, point_1.get_y());
                let is_new_point = observed_points.insert(point);
                if !is_new_point {
                    overlap_points.insert(point);
                }
            }
        // Otherwise, the line is a diagonal
        } else {
            // Generate the co-ordinate pairs describing the diagonal line
            let delta_x = (point_2.get_x() - point_1.get_x()).signum();
            let delta_y = (point_2.get_y() - point_1.get_y()).signum();
            let mut point = point_1.clone();
            while point.get_x() != point_2.get_x() + delta_x
                && point.get_y() != point_2.get_y() + delta_y
            {
                let is_new_point = observed_points.insert(point);
                if !is_new_point {
                    overlap_points.insert(point);
                }
                point.move_point(delta_x, delta_y);
            }
        }
    }
    return overlap_points.len();
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
