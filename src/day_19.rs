use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Vec<Vec<(i64, i64, i64)>> {
    let mut scanner_data: Vec<Vec<(i64, i64, i64)>> = vec![];
    let mut input_lines = input.lines();
    loop {
        // Process next scanner data input
        let scanner_header = input_lines.next();
        if scanner_header.is_none() {
            break;
        }
        let mut beacons: Vec<(i64, i64, i64)> = vec![];
        loop {
            let next = {
                let candidate = input_lines.next();
                if candidate.is_none() {
                    break;
                }
                candidate.unwrap().trim()
            };
            if next.is_empty() {
                break;
            }
            let beacon_pos = next
                .split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            beacons.push((beacon_pos[0], beacon_pos[1], beacon_pos[2]));
        }
        scanner_data.push(beacons);
    }
    return scanner_data;
}

#[aoc(day19, part1)]
fn solve_part_1(scanner_data: &Vec<Vec<(i64, i64, i64)>>) -> usize {
    let (scanner_locations_abs, beacon_relvecs_abs) = process_scanner_data(scanner_data);
    // Determine number of unique beacons
    let mut unique_beacons: HashSet<(i64, i64, i64)> = HashSet::new();
    for (scanner_i, scanner_loc) in scanner_locations_abs.iter() {
        // Relative vectors for each scanner are now in the absolute FoR
        let rel_vecs = beacon_relvecs_abs.get(scanner_i).unwrap();
        for rel_vec in rel_vecs {
            let beacon_loc = add_vectors(scanner_loc, &rel_vec);
            unique_beacons.insert(beacon_loc);
        }
    }
    return unique_beacons.len();
}

#[aoc(day19, part2)]
fn solve_part_2(scanner_data: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    let mut largest_m_dist = 0;
    let (scanner_locations_abs, _beacon_relvecs_abs) = process_scanner_data(scanner_data);
    for i in 0..scanner_data.len() {
        for j in (i + 1)..scanner_data.len() {
            let left = scanner_locations_abs.get(&i).unwrap();
            let right = scanner_locations_abs.get(&j).unwrap();
            let m_dist =
                (left.0 - right.0).abs() + (left.1 - right.1).abs() + (left.2 - right.2).abs();
            if m_dist > largest_m_dist {
                largest_m_dist = m_dist;
            }
        }
    }
    return largest_m_dist;
}

/// Processes the given scanner data to find the absolute locations of each scanner, and the
/// relative vector for each beacon to the scanner that detected it in the absolute
/// frame-of-reference.
fn process_scanner_data(
    scanner_data: &Vec<Vec<(i64, i64, i64)>>,
) -> (
    HashMap<usize, (i64, i64, i64)>,
    HashMap<usize, Vec<(i64, i64, i64)>>,
) {
    // Track scanners with known locations
    let mut scanner_locations_abs: HashMap<usize, (i64, i64, i64)> = HashMap::new();
    scanner_locations_abs.insert(0, (0, 0, 0));
    // Track which scanners to use next as known source
    let mut scanners_for_source: VecDeque<usize> = VecDeque::new();
    scanners_for_source.push_back(0);
    // Track which scanners do not yet have a known position
    let mut scanners_unlocated: HashSet<usize> = HashSet::new();
    for i in 1..scanner_data.len() {
        scanners_unlocated.insert(i);
    }
    // For each scanner, calculate relative vectors of each beacon to others detected by scanner
    let mut scanner_constellations: Vec<HashMap<(i64, i64, i64), HashSet<(i64, i64, i64)>>> =
        vec![];
    for i in 0..scanner_data.len() {
        let constellation = calculate_scanner_constellation(&scanner_data[i]);
        scanner_constellations.push(constellation);
    }
    // Try to find overlaps
    let rot_seq = generate_rotation_sequence();
    loop {
        // Halt when all scanners have had location determined
        if scanners_for_source.is_empty() {
            break;
        }
        // From current source scanner, find all other scanners that overlap with its detect cube
        let src_scanner_index = scanners_for_source.pop_front().unwrap();
        let src_scanner_loc = scanner_locations_abs
            .get(&src_scanner_index)
            .unwrap()
            .clone();
        let mut scanners_located: Vec<usize> = vec![];
        for dest_scanner_index in scanners_unlocated.iter() {
            // Try the 24 different orientations
            let mut overlap_points: Vec<((i64, i64, i64), (i64, i64, i64))> = vec![];
            for i in 0..24 {
                overlap_points = vec![];
                // Try overlap
                for (dest_beacon_loc, dest_rel_vecs) in
                    scanner_constellations[*dest_scanner_index].iter()
                {
                    for (src_beacon_loc, src_rel_vecs) in
                        scanner_constellations[src_scanner_index].iter()
                    {
                        if dest_rel_vecs.intersection(src_rel_vecs).count() >= 11 {
                            overlap_points.push((*src_beacon_loc, *dest_beacon_loc));
                        }
                    }
                }
                // Overlap occurs if at least 12 beacons are found with matching constellation
                if overlap_points.len() >= 12 {
                    break;
                }
                // Apply rotation sequence
                for rot_fn in rot_seq[i].iter() {
                    let new_constellation =
                        rotate_constellation(*rot_fn, &scanner_constellations[*dest_scanner_index]);
                    scanner_constellations[*dest_scanner_index] = new_constellation;
                }
            }
            if overlap_points.len() >= 12 {
                // Add destination scanner to end of source scanner queue
                scanners_for_source.push_back(*dest_scanner_index);
                scanners_located.push(*dest_scanner_index);
                // Calculate absolute position of destination scanner
                let rel_vec =
                    &calculate_relative_vector(&overlap_points[0].1, &overlap_points[0].0);
                let dest_abs_loc = add_vectors(&src_scanner_loc, &rel_vec);
                scanner_locations_abs.insert(*dest_scanner_index, dest_abs_loc);
            }
        }
        // Remove the located scanner from the unlocated collection
        for loc in scanners_located {
            scanners_unlocated.remove(&loc);
        }
    }
    // Extract abs FoR beacon locations for each scanner
    let mut beacon_locations_abs: HashMap<usize, Vec<(i64, i64, i64)>> = HashMap::new();
    for scanner_i in 0..scanner_constellations.len() {
        for loc in scanner_constellations[scanner_i].keys() {
            beacon_locations_abs
                .entry(scanner_i)
                .or_insert(vec![])
                .push(*loc);
        }
    }
    return (scanner_locations_abs, beacon_locations_abs);
}

/// Uses the given beacon locations to calculate the relative vectors for each beacon to all of the
/// other beacons. The result represents the constellation for the scanner which detected the given
/// beacon locations.
fn calculate_scanner_constellation(
    locs: &Vec<(i64, i64, i64)>,
) -> HashMap<(i64, i64, i64), HashSet<(i64, i64, i64)>> {
    let mut result: HashMap<(i64, i64, i64), HashSet<(i64, i64, i64)>> = HashMap::new();
    for src in 0..locs.len() {
        let mut rel_vecs: HashSet<(i64, i64, i64)> = HashSet::new();
        for dest in 0..locs.len() {
            if src == dest {
                continue;
            }
            let rel = (
                locs[dest].0 - locs[src].0,
                locs[dest].1 - locs[src].1,
                locs[dest].2 - locs[src].2,
            );
            rel_vecs.insert(rel);
        }
        result.insert(locs[src], rel_vecs);
    }
    return result;
}

/// Subtracts the source vector from the destination vector, providing the relative vector pointing
/// from the source to the destination location.
fn calculate_relative_vector(src: &(i64, i64, i64), dest: &(i64, i64, i64)) -> (i64, i64, i64) {
    return (dest.0 - src.0, dest.1 - src.1, dest.2 - src.2);
}

/// Adds the two given vectors together.
fn add_vectors(src: &(i64, i64, i64), dest: &(i64, i64, i64)) -> (i64, i64, i64) {
    return (src.0 + dest.0, src.1 + dest.1, src.2 + dest.2);
}

/// Generates the sequence of basic rotations required to move a scanner's frame-of-reference
/// through all 24 possible axial and rotational orientation possibilities.
fn generate_rotation_sequence() -> Vec<Vec<fn(&(i64, i64, i64)) -> (i64, i64, i64)>> {
    let mut rot_seq: Vec<Vec<fn(&(i64, i64, i64)) -> (i64, i64, i64)>> = vec![];
    // +x
    rot_seq.push(vec![rot90_x_axis]);
    rot_seq.push(vec![rot90_x_axis]);
    rot_seq.push(vec![rot90_x_axis]);
    rot_seq.push(vec![rot90_x_axis, rot90_z_axis]);
    // +y
    rot_seq.push(vec![rot90_y_axis]);
    rot_seq.push(vec![rot90_y_axis]);
    rot_seq.push(vec![rot90_y_axis]);
    rot_seq.push(vec![rot90_y_axis, rot90_z_axis]);
    // -x
    rot_seq.push(vec![rot90_x_axis]);
    rot_seq.push(vec![rot90_x_axis]);
    rot_seq.push(vec![rot90_x_axis]);
    rot_seq.push(vec![rot90_x_axis, rot90_z_axis]);
    // -y
    rot_seq.push(vec![rot90_y_axis]);
    rot_seq.push(vec![rot90_y_axis]);
    rot_seq.push(vec![rot90_y_axis]);
    rot_seq.push(vec![rot90_y_axis, rot90_x_axis]);
    // -z
    rot_seq.push(vec![rot90_z_axis]);
    rot_seq.push(vec![rot90_z_axis]);
    rot_seq.push(vec![rot90_z_axis]);
    rot_seq.push(vec![rot90_z_axis, rot90_x_axis, rot90_x_axis]);
    // +z
    rot_seq.push(vec![rot90_z_axis]);
    rot_seq.push(vec![rot90_z_axis]);
    rot_seq.push(vec![rot90_z_axis]);
    rot_seq.push(vec![rot90_z_axis, rot90_y_axis]);
    return rot_seq;
}

/// Applies the given basic rotation function to the given scanner constellation.
fn rotate_constellation(
    rot_fn: fn(&(i64, i64, i64)) -> (i64, i64, i64),
    constellation: &HashMap<(i64, i64, i64), HashSet<(i64, i64, i64)>>,
) -> HashMap<(i64, i64, i64), HashSet<(i64, i64, i64)>> {
    let mut new_result: HashMap<(i64, i64, i64), HashSet<(i64, i64, i64)>> = HashMap::new();
    for (src_loc, rel_vecs) in constellation {
        let new_src_loc = rot_fn(&src_loc);
        let mut new_rel_vecs: HashSet<(i64, i64, i64)> = HashSet::new();
        for vec in rel_vecs {
            new_rel_vecs.insert(rot_fn(&vec));
        }
        new_result.insert(new_src_loc, new_rel_vecs);
    }
    return new_result;
}

/// Performs a basic rotation of the input co-ordinates by 90 degrees about x-axis (using RHR).
fn rot90_x_axis(input: &(i64, i64, i64)) -> (i64, i64, i64) {
    return (input.0, -input.2, input.1);
}

/// Performs a basic rotation of the input co-ordinates by 90 degrees about y-axis (using RHR).
fn rot90_y_axis(input: &(i64, i64, i64)) -> (i64, i64, i64) {
    return (input.2, input.1, -input.0);
}

/// Performs a basic rotation of the input co-ordinates by 90 degrees about z-axis (using RHR).
fn rot90_z_axis(input: &(i64, i64, i64)) -> (i64, i64, i64) {
    return (-input.1, input.0, input.2);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d19_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day19.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(459, result);
    }

    #[test]
    fn test_d19_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day19.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(19130, result);
    }
}
