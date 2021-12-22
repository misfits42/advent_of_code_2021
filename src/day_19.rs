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
    let mut scanner_data = scanner_data.clone();
    // Track scanners with known locations
    let mut scanner_locations: HashMap<usize, (i64, i64, i64)> = HashMap::new();
    scanner_locations.insert(0, (0, 0, 0));
    // Track which scanners to use next as known source
    let mut scanners_for_source: VecDeque<usize> = VecDeque::new();
    scanners_for_source.push_back(0);
    // Track which scanners do not yet have a known position
    let mut scanners_to_locate: HashSet<usize> = HashSet::new();
    for i in 1..scanner_data.len() {
        scanners_to_locate.insert(i);
    }
    // Record beacons from scanner 0 as having known absolute positions
    // let mut beacon_loc_abs: HashMap<usize, HashSet<(i64, i64, i64)>> = HashMap::new();
    // for loc in scanner_data[0].iter() {
    //     beacon_loc_abs
    //         .entry(0)
    //         .or_insert(HashSet::new())
    //         .insert(*loc);
    // }
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
        let src_scanner_index = scanners_for_source.pop_front().unwrap();
        let src_scanner_loc = scanner_locations.get(&src_scanner_index).unwrap().clone();
        let mut scanners_located: Vec<usize> = vec![];
        for dest_scanner_index in scanners_to_locate.iter() {
            // Try the 24 different orientations
            // let mut overlap_found = false;
            let mut overlap_points: Vec<((i64, i64, i64), (i64, i64, i64))> = vec![];
            for i in 0..24 {
                overlap_points = vec![];
                let mut new_scanner_data: Vec<(i64, i64, i64)> = vec![];
                // Try overlap
                for (dest_beacon_loc, dest_rel_vecs) in
                    scanner_constellations[*dest_scanner_index].iter()
                {
                    new_scanner_data.push(*dest_beacon_loc);
                    for (src_beacon_loc, src_rel_vecs) in scanner_constellations[src_scanner_index].iter() {
                        if dest_rel_vecs.intersection(src_rel_vecs).count() >= 11 {
                            // Rotate the destination point back to original value
                            let mut dest_beacon_loc_orig = *dest_beacon_loc;
                            for j in i..rot_seq.len() {
                                for rot_fn in rot_seq[j].iter() {
                                    dest_beacon_loc_orig = rot_fn(&dest_beacon_loc_orig);
                                }
                            }
                            overlap_points.push((*src_beacon_loc, dest_beacon_loc_orig));
                        }
                    }
                }
                scanner_data[*dest_scanner_index] = new_scanner_data;
                // The two scanner regions overlap if at least 12 beacons are found with matching constellation
                if overlap_points.len() >= 12 {
                    // Rotate back to original FoR
                    // for j in i..rot_seq.len() {
                    //     for rot_fn in rot_seq[j].iter() {
                    //         let new_constellation = rotate_constellation(*rot_fn, &scanner_constellations[*dest_scanner_index]);
                    //         scanner_constellations[*dest_scanner_index] = new_constellation;
                    //     }
                    // }
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
                let rel_vec = &calculate_relative_vector(&overlap_points[0].1, &overlap_points[0].0);
                let dest_abs_loc = add_vectors(&src_scanner_loc, &rel_vec);
                scanner_locations.insert(*dest_scanner_index, dest_abs_loc);


                // Update locations of beacons from destination scanner into the absolute FoR
                let dest_beacons_abs_locs = shift_beacon_locations(&scanner_data[*dest_scanner_index], &rel_vec);
                scanner_data[*dest_scanner_index] = dest_beacons_abs_locs.iter().map(|x| *x).collect::<Vec<(i64, i64, i64)>>();
                //
                println!("####################");
                println!("## Overlaps with scanner {} // {:?} // rel scanner {}", *dest_scanner_index, dest_abs_loc, src_scanner_index);
                let overlap_points_rel_src = overlap_points.iter().map(|x| x.0).collect::<Vec<(i64, i64, i64)>>();
                for loc in overlap_points_rel_src.iter() {
                    println!(">>>> {:?}", loc);
                }
                println!("## Overlaps with scanner {} // {:?} // rel scanner {}", *dest_scanner_index, dest_abs_loc, *dest_scanner_index);
                let overlap_points_rel_src = overlap_points.iter().map(|x| x.1).collect::<Vec<(i64, i64, i64)>>();
                for loc in overlap_points_rel_src.iter() {
                    println!(">>>> {:?}", loc);
                }
                println!("## absolute locations of beacons from scanner {}", *dest_scanner_index);
                for loc in dest_beacons_abs_locs.iter() {
                    println!(">>>> {:?}", loc);
                }
                // // beacon_loc_abs.insert(*dest_scanner_index, dest_beacons_abs_locs);
            }
        }
        for loc in scanners_located {
            scanners_to_locate.remove(&loc);
        }
    }
    println!("Scanner locations: {:?}", scanner_locations);
    // Determine number of unique beacons
    let mut unique_beacons: HashSet<(i64, i64, i64)> = HashSet::new();
    for scanner_i in 0..scanner_data.len() {
        let scanner_loc = scanner_locations.get(&scanner_i).unwrap();
        for beacon_loc_rel in scanner_data[scanner_i].iter() {
            let beacon_loc_abs = subtract_vectors(beacon_loc_rel, scanner_loc);
            unique_beacons.insert(beacon_loc_abs);
        }
    }
    // for beacon_locs in beacon_loc_abs.values() {
    //     unique_beacons = unique_beacons.union(beacon_locs).map(|x| *x).collect();
    // }
    return unique_beacons.len();
}

fn subtract_vectors(left: &(i64, i64, i64), right: &(i64, i64, i64)) -> (i64, i64, i64) {
    return (left.0 - right.0, left.1 - right.1, left.2 - right.2);
}

fn shift_beacon_locations(beacon_locs: &Vec<(i64, i64, i64)>, rel_vec: &(i64, i64, i64)) -> HashSet<(i64, i64, i64)> {
    let mut abs_locs: HashSet<(i64, i64, i64)> = HashSet::new();
    for loc in beacon_locs {
        abs_locs.insert(add_vectors(rel_vec, loc));
    }
    return abs_locs;
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

fn flip_vector_signs(src: &(i64, i64, i64)) -> (i64, i64, i64) {
    return (-src.0, -src.1, -src.2);
}

fn calculate_relative_vector(src: &(i64, i64, i64), dest: &(i64, i64, i64)) -> (i64, i64, i64) {
    return (dest.0 - src.0, dest.1 - src.1, dest.2 - src.2);
}

fn add_vectors(loc: &(i64, i64, i64), vec: &(i64, i64, i64)) -> (i64, i64, i64) {
    return (loc.0 + vec.0, loc.1 + vec.1, loc.2 + vec.2);
}

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
        unimplemented!();
    }
}
