use std::collections::HashSet;

use super::utils::map::Point2D;

#[aoc_generator(day13)]
fn parse_input(input: &str) -> (HashSet<Point2D>, Vec<Point2D>) {
    let mut points_map: HashSet<Point2D> = HashSet::new();
    let mut fold_instructions: Vec<Point2D> = vec![];
    let mut points_map_check = true;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            points_map_check = false;
            continue;
        }
        if points_map_check {
            let coords = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            points_map.insert(Point2D::new(coords[0], coords[1]));
        } else {
            if line.contains("x") {
                let split = line.split("=").collect::<Vec<&str>>();
                let coord = split[1].parse::<i64>().unwrap();
                fold_instructions.push(Point2D::new(coord, -1));
            } else { // line.contains("y")
                let split = line.split("=").collect::<Vec<&str>>();
                let coord = split[1].parse::<i64>().unwrap();
                fold_instructions.push(Point2D::new(-1, coord));
            }
        }
    }
    return (points_map, fold_instructions);
}

#[aoc(day13, part1)]
fn solve_part_1(dot_instructions: &(HashSet<Point2D>, Vec<Point2D>)) -> usize {
    // Apply only the first fold instruction
    let dot_locs = &dot_instructions.0;
    let fold_instr = &dot_instructions.1;
    let mut new_dot_locs: HashSet<Point2D> = HashSet::new();
    for dot_loc in dot_locs.iter() {
        // x-fold instruction
        if fold_instr[0].get_x() > 0 {
            // Dot is not on the half being folded up
            if dot_loc.get_x() <= fold_instr[0].get_x() {
                new_dot_locs.insert(*dot_loc);
            } else {
                // Calculate new x
                let delta_x = dot_loc.get_x() - fold_instr[0].get_x();
                let new_dot = Point2D::new(fold_instr[0].get_x() - delta_x, dot_loc.get_y());
                new_dot_locs.insert(new_dot);
            }
        // y-fold instruction
        } else {
            // Dot is not on the half being folded up
            if dot_loc.get_y() <= fold_instr[0].get_y() {
                new_dot_locs.insert(*dot_loc);
            } else {
                // Calculate new y
                let delta_y = dot_loc.get_y() - fold_instr[0].get_y();
                let new_dot = Point2D::new(dot_loc.get_y(), fold_instr[0].get_y() - delta_y);
                new_dot_locs.insert(new_dot);
            }
        }
    }
    return new_dot_locs.len();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d13_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day13.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(607, result);
    }
}
