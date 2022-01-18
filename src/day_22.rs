use std::collections::HashSet;
use std::collections::VecDeque;

use regex::Regex;

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
struct Cuboid {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

impl Cuboid {
    pub fn new(x_min: i64, x_max: i64, y_min: i64, y_max: i64, z_min: i64, z_max: i64) -> Self {
        Self {
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max,
            z_min: z_min,
            z_max: z_max,
        }
    }

    pub fn calculate_volume(&self) -> usize {
        let mut total_volume: usize = 1;
        total_volume *= (self.x_max - self.x_min + 1) as usize;
        total_volume *= (self.y_max - self.y_min + 1) as usize;
        total_volume *= (self.z_max - self.z_min + 1) as usize;
        return total_volume;
    }
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Vec<(bool, Cuboid)> {
    let mut reboot_commands: Vec<(bool, Cuboid)> = vec![];
    let line_regex =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let captures = line_regex.captures(line).unwrap();
        let on_state = {
            if &captures[1] == "on" {
                true
            } else {
                false
            }
        };
        let x_min = captures[2].parse::<i64>().unwrap();
        let x_max = captures[3].parse::<i64>().unwrap();
        let y_min = captures[4].parse::<i64>().unwrap();
        let y_max = captures[5].parse::<i64>().unwrap();
        let z_min = captures[6].parse::<i64>().unwrap();
        let z_max = captures[7].parse::<i64>().unwrap();
        let cmd = (
            on_state,
            Cuboid::new(x_min, x_max, y_min, y_max, z_min, z_max),
        );
        reboot_commands.push(cmd);
    }
    return reboot_commands;
}

#[aoc(day22, part1)]
fn solve_part_1(reboot_commands: &Vec<(bool, Cuboid)>) -> usize {
    let mut reactor_cubes_on: HashSet<(i64, i64, i64)> = HashSet::new();
    let commands_filtered: Vec<(bool, Cuboid)> = reboot_commands
        .iter()
        .map(|x| *x)
        .filter(|v| {
            v.1.x_min >= -50
                && v.1.x_max <= 50
                && v.1.y_min >= -50
                && v.1.y_max <= 50
                && v.1.z_min >= -50
                && v.1.z_max <= 50
        })
        .collect();
    for (on_state, cube) in commands_filtered {
        for z in cube.z_min..=cube.z_max {
            for y in cube.y_min..=cube.y_max {
                for x in cube.x_min..=cube.x_max {
                    if on_state {
                        reactor_cubes_on.insert((x, y, z));
                    } else {
                        reactor_cubes_on.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    return reactor_cubes_on.len();
}
fn determine_positive_space_cuboids(inner_cuboid: &Cuboid, outer_cuboid: &Cuboid) -> Vec<Cuboid> {
    let mut new_cuboids: Vec<Cuboid> = vec![];
    // Check for nil overlap
    if (outer_cuboid.x_min < inner_cuboid.x_min && outer_cuboid.x_max < inner_cuboid.x_min
        || outer_cuboid.x_min > inner_cuboid.x_max && outer_cuboid.x_max > inner_cuboid.x_max)
        || (outer_cuboid.y_min < inner_cuboid.y_min && outer_cuboid.y_max < inner_cuboid.y_min
            || outer_cuboid.y_min > inner_cuboid.y_max && outer_cuboid.y_max > inner_cuboid.y_max)
        || (outer_cuboid.z_min < inner_cuboid.z_min && outer_cuboid.z_max < inner_cuboid.z_min
            || outer_cuboid.z_min > inner_cuboid.z_max && outer_cuboid.z_max > inner_cuboid.z_max)
    {
        return vec![];
    }
    // Determine overlap points with extant cube
    // x
    let ocube_x_min = {
        if outer_cuboid.x_min <= inner_cuboid.x_min {
            inner_cuboid.x_min
        } else {
            outer_cuboid.x_min
        }
    };
    let ocube_x_max = {
        if inner_cuboid.x_max <= outer_cuboid.x_max {
            inner_cuboid.x_max
        } else {
            outer_cuboid.x_max
        }
    };
    // y
    let ocube_y_min = {
        if outer_cuboid.y_min <= inner_cuboid.y_min {
            inner_cuboid.y_min
        } else {
            outer_cuboid.y_min
        }
    };
    let ocube_y_max = {
        if inner_cuboid.y_max <= outer_cuboid.y_max {
            inner_cuboid.y_max
        } else {
            outer_cuboid.y_max
        }
    };
    // z
    let ocube_z_min = {
        if outer_cuboid.z_min <= inner_cuboid.z_min {
            inner_cuboid.z_min
        } else {
            outer_cuboid.z_min
        }
    };
    let ocube_z_max = {
        if inner_cuboid.z_max <= outer_cuboid.z_max {
            inner_cuboid.z_max
        } else {
            outer_cuboid.z_max
        }
    };
    // println!("$$$$$$$$$$");
    // println!(
    //     "ocube - x: {},{} // y: {},{} // z: {},{}",
    //     ocube_x_min, ocube_x_max, ocube_y_min, ocube_y_max, ocube_z_min, ocube_z_max
    // );
    // println!("$$$$$$$$$$");
    // Calculate the six new cuboids to add
    // z pos
    if outer_cuboid.z_max > ocube_z_max {
        // println!("z pos");
        new_cuboids.push(Cuboid::new(
            outer_cuboid.x_min,
            outer_cuboid.x_max,
            outer_cuboid.y_min,
            outer_cuboid.y_max,
            ocube_z_max + 1,
            outer_cuboid.z_max,
        ));
    }
    // z neg
    if ocube_z_min > outer_cuboid.z_min {
        // println!("z neg");
        new_cuboids.push(Cuboid::new(
            outer_cuboid.x_min,
            outer_cuboid.x_max,
            outer_cuboid.y_min,
            outer_cuboid.y_max,
            outer_cuboid.z_min,
            ocube_z_min - 1,
        ));
    }
    // y pos
    if outer_cuboid.y_max > ocube_y_max {
        // println!("y pos");
        new_cuboids.push(Cuboid::new(
            outer_cuboid.x_min,
            outer_cuboid.x_max,
            ocube_y_max + 1,
            outer_cuboid.y_max,
            ocube_z_min,
            ocube_z_max,
        ));
    }
    // y neg
    if ocube_y_min > outer_cuboid.y_min {
        //println!("y neg");
        new_cuboids.push(Cuboid::new(
            outer_cuboid.x_min,
            outer_cuboid.x_max,
            outer_cuboid.y_min,
            ocube_y_min - 1,
            ocube_z_min,
            ocube_z_max,
        ));
    }
    // x pos
    if outer_cuboid.x_max > ocube_x_max {
        //println!("x pos");
        new_cuboids.push(Cuboid::new(
            ocube_x_max + 1,
            outer_cuboid.x_max,
            ocube_y_min,
            ocube_y_max,
            ocube_z_min,
            ocube_z_max,
        ));
    }
    // x neg
    if ocube_x_min > outer_cuboid.x_min {
        //println!("x neg");
        new_cuboids.push(Cuboid::new(
            outer_cuboid.x_min,
            ocube_x_min - 1,
            ocube_y_min,
            ocube_y_max,
            ocube_z_min,
            ocube_z_max,
        ));
    }
    // println!("overlap cuboids: {:?}", new_cuboids);
    // println!("$$$$$$$$$$");
    return new_cuboids;
}

#[aoc(day22, part2)]
fn solve_part_2(reboot_commands: &Vec<(bool, Cuboid)>) -> usize {
    let mut reactor_cuboids_on: Vec<Cuboid> = vec![];    
    for (on_state, new_cuboid) in reboot_commands.iter() {
        println!("no. reactor cuboids: {}", reactor_cuboids_on.len());
        println!("!!!!! on_state: {} // new cuboid: {:?}", on_state, new_cuboid);
        let mut total_volume = 0;
        for cuboid in reactor_cuboids_on.iter() {
            total_volume += cuboid.calculate_volume();
        }
        println!(">>>> volume: {}", total_volume);
        if *on_state {
            // Track the resulting cuboids from overlap of previous state and current command
            let mut new_cuboids: Vec<Cuboid> = vec![];
            if reactor_cuboids_on.is_empty() {
                reactor_cuboids_on.push(*new_cuboid);
                continue;
            }
            for extant_cuboid in reactor_cuboids_on.iter() {
                // println!("@@ new cuboids len: {}", new_cuboids.len());
                let mut current_new_cuboids =
                    determine_positive_space_cuboids(extant_cuboid, new_cuboid);
                // If no overlap between extant and new cuboids, new cuboid is added in entirity
                if current_new_cuboids.is_empty() {
                    current_new_cuboids.push(*new_cuboid);
                }
                new_cuboids.append(&mut current_new_cuboids);
            }
            for cuboid in new_cuboids {
                reactor_cuboids_on.push(cuboid);
            }
            // // Now sort out overlaps in the new cuboids
            // let mut updated_cuboids: Vec<Cuboid> = vec![];
            // let mut overlapping_new_cuboids: HashSet<Cuboid> = HashSet::new();
            // for i in 0..new_cuboids.len() {
            //     let mut no_overlaps = true;
            //     for j in (i + 1)..new_cuboids.len() {
            //         let overlaps = determine_positive_space_cuboids(&new_cuboids[i], &new_cuboids[j]);
            //         if !overlaps.is_empty() {
            //             no_overlaps = false;
            //             overlapping_new_cuboids.insert(new_cuboids[i]);
            //             overlapping_new_cuboids.insert(new_cuboids[j]);
            //             break;
            //         }
            //     }
            //     if no_overlaps {
            //         updated_cuboids.push(new_cuboids[i]);
            //     }
            // }
            // let mut cuboids_merge_queue: VecDeque<Cuboid> = overlapping_new_cuboids.iter().map(|x| *x).collect();
            // let mut cuboids_to_ignore: HashSet<Cuboid> = HashSet::new();
            // loop {
            //     println!("cuboid merging - queue len: {}", cuboids_merge_queue.len());
            //     if cuboids_merge_queue.is_empty() {
            //         break;
            //     }
            //     let current = cuboids_merge_queue.pop_front().unwrap();
            //     if cuboids_to_ignore.contains(&current) {
            //         continue;
            //     }
            //     updated_cuboids.push(current);
            //     let mut cuboids_to_queue: HashSet<Cuboid> = HashSet::new();
            //     for cuboid in cuboids_merge_queue.iter() {
            //         let overlaps = determine_positive_space_cuboids(&current, &cuboid);
            //         if overlaps.is_empty() {
            //             continue;
            //         } else {
            //             cuboids_to_ignore.insert(*cuboid);
            //             for overlap_cuboid in overlaps {
            //                 cuboids_to_queue.insert(overlap_cuboid);
            //             }
            //         }
            //     }
            //     for cuboid in cuboids_to_queue {
            //         cuboids_merge_queue.push_back(cuboid);
            //     }
            // }
            // new_cuboids = updated_cuboids;
            // for cube in new_cuboids {
            //     reactor_cuboids_on.push(cube);
            // }

            // // // Now sort out overlaps in the new cuboids
            // // let mut temp: HashSet<Cuboid> = HashSet::new();
            // // for i in 0..new_cuboids.len() {
            // //     if temp.contains(&new_cuboids[i]) {
            // //         continue;
            // //     }
            // //     for j in (i + 1)..new_cuboids.len() {
            // //         let overlaps = determine_positive_space_cuboids(&new_cuboids[i], &new_cuboids[j]);
            // //         if overlaps.is_empty() {
            // //             //println!("no overlap at i={} j={}", i, j);
            // //             temp.insert(new_cuboids[i]);
            // //             temp.insert(new_cuboids[j]);
            // //         } else {
            // //             //println!("overlap at i={} j={} // overlaps len: {}", i, j, overlaps.len());
            // //             for cuboid in overlaps {
            // //                 temp.insert(cuboid);
            // //             }
            // //         }
            // //     }
            // // }
            // // let cuboids_to_check: Vec<Cuboid> = temp.iter().map(|x| *x).collect();
            // // let mut updated_cuboids: Vec<Cuboid> = vec![];
            // // for i in 0..cuboids_to_check.len() {
            // //     let mut no_overlap = true;
            // //     for j in (i + 1)..cuboids_to_check.len() {
            // //         let overlaps = determine_positive_space_cuboids(&cuboids_to_check[i], &cuboids_to_check[j]);
            // //         if !overlaps.is_empty() {
            // //             no_overlap = false;
            // //             break;
            // //         }
            // //     }
            // //     if no_overlap {
            // //         updated_cuboids.push(cuboids_to_check[i]);
            // //     }
            // // }
            // // // // std::thread::sleep(std::time::Duration::from_millis(100));
        } else {
            let mut new_cuboids: Vec<Cuboid> = vec![];
            if reactor_cuboids_on.is_empty() {
                continue;
            }
            for extant_cuboid in reactor_cuboids_on.iter() {
                // Determine components of extant cuboid if new cuboid is subtracted
                let mut current_new_cuboids =
                    determine_positive_space_cuboids(new_cuboid, extant_cuboid);
                // println!("## Extant cuboid: {:?}", extant_cuboid);
                // println!("## New cuboid: {:?}", extant_cuboid);
                // println!("##### Current new cuboids: {:?}", current_new_cuboids);
                // If there is no overlap, then the extant cuboid is retained in entirity
                if current_new_cuboids.is_empty() {
                    new_cuboids.push(*extant_cuboid);
                }
                new_cuboids.append(&mut current_new_cuboids);
                // let mut overlap_new_cuboids: Vec<Cuboid> = vec![];
                // //if !new_cuboids.is_empty() {
                // for cube_1 in current_new_cuboids.iter() {
                //     for cube_2 in new_cuboids.iter() {
                //         let mut overlaps = determine_positive_space_cuboids(cube_1, cube_2);
                //         overlap_new_cuboids.append(&mut overlaps);
                //     }
                // }
                // new_cuboids = overlap_new_cuboids;
                // // } else {
                // //     new_cuboids = current_new_cuboids;
                // // }
            }
            reactor_cuboids_on = new_cuboids;
        }
    }
    // println!("%%%% Reactor cubes on: {:?}", reactor_cuboids_on);
    // Calculate total volume of reactor cubes switched on
    let mut total_volume = 0;
    for cube in reactor_cuboids_on.iter() {
        total_volume += cube.calculate_volume();
    }
    return total_volume;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d22_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day22.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(615700, result);
    }
}
