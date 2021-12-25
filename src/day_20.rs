use std::cmp::Ordering;
use std::collections::HashMap;

#[aoc_generator(day20)]
fn parse_input(input: &str) -> (Vec<char>, HashMap<(i64, i64), char>, (i64, i64), (i64, i64)) {
    let mut input_lines = input.lines();
    // Read in the input enhancement algorithm
    let mut img_enhance_alg_raw = input_lines.next().unwrap().trim().to_string();
    img_enhance_alg_raw = img_enhance_alg_raw.replace(".", "0");
    img_enhance_alg_raw = img_enhance_alg_raw.replace("#", "1");
    let img_enhance_alg = img_enhance_alg_raw.chars().collect::<Vec<char>>();
    input_lines.next();
    // Read in the input image
    let mut input_image: HashMap<(i64, i64), char> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    loop {
        let mut line = {
            let candidate = input_lines.next();
            if candidate.is_none() {
                break;
            }
            candidate.unwrap().trim().to_string()
        };
        line = line.replace(".", "0");
        line = line.replace("#", "1");
        x = 0;
        for c in line.chars() {
            input_image.insert((x, y), c);
            x += 1;
        }
        y += 1;
    }
    // Pad out extra three layers around input image
    let mut x_min = -3;
    let mut x_max = x + 2;
    let mut y_min = -3;
    let mut y_max = y + 2;
    for _ in 0..3 {
        for x_new in x_min..=x_max {
            input_image.insert((x_new, y_min), '0');
            input_image.insert((x_new, y_max), '0');
        }
        for y_new in y_min..=y_max {
            input_image.insert((x_min, y_new), '0');
            input_image.insert((x_max, y_new), '0');
        }
        x_min += 1;
        x_max -= 1;
        y_min += 1;
        y_max -= 1;
    }
    return (img_enhance_alg, input_image, (-3, -3), (x + 2, y + 2));
}

#[aoc(day20, part1)]
fn solve_part_1(
    image_input: &(Vec<char>, HashMap<(i64, i64), char>, (i64, i64), (i64, i64)),
) -> usize {
    let img_enhance_alg = image_input.0.clone();
    let mut input_image = image_input.1.clone();
    // let mut x_lim = image_input.2;
    // let mut y_lim = image_input.3;
    let (mut x_min, mut y_min) = image_input.2;
    let (mut y_max, mut x_max) = image_input.3;
    let mut default_c = *input_image.get(&(x_min, y_min)).unwrap();
    let mut output_image: HashMap<(i64, i64), char> = HashMap::new();
    for _ in 0..2 {
        // Apply first iteration
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                // Determine points surrounding current location
                let pos = (x, y);
                let mut surround_points: Vec<(i64, i64)> = vec![];
                for delta_y in -1..=1 {
                    for delta_x in -1..=1 {
                        surround_points.push((pos.0 + delta_x, pos.1 + delta_y));
                    }
                }
                surround_points.sort_by(|a, b| {
                    if a.1 < b.1 {
                        return Ordering::Less;
                    }
                    if a.1 == b.1 && a.0 < b.0 {
                        return Ordering::Less;
                    }
                    if a.1 == b.1 && a.0 > b.0 {
                        return Ordering::Greater;
                    }
                    if a.1 > b.1 {
                        return Ordering::Greater;
                    }
                    return Ordering::Equal;
                });
                // Determine img enhance alg index
                let mut index_string = String::new();
                for pos_s in surround_points.iter() {
                    let c = *input_image.get(&pos_s).unwrap_or(&default_c);
                    index_string.push(c);
                }
                let index = usize::from_str_radix(&index_string, 2).unwrap();
                output_image.insert((x, y), img_enhance_alg[index]);
            }
        }
        // Pad out another layer
        default_c = *output_image.get(&(x_min, y_min)).unwrap();
        x_min -= 1;
        y_min -= 1;
        x_max += 1;
        y_max += 1;
        for x_new in x_min..=x_max {
            input_image.insert((x_new, y_min), default_c);
            input_image.insert((x_new, y_max), default_c);
        }
        for y_new in y_min..=y_max {
            input_image.insert((x_min, y_new), default_c);
            input_image.insert((x_max, y_new), default_c);
        }
        // Apply next iteration
        input_image = output_image;
        output_image = HashMap::new();
    }
    return input_image.values().filter(|x| **x == '1').count();
}

#[aoc(day20, part2)]
fn solve_part_2(
    image_input: &(Vec<char>, HashMap<(i64, i64), char>, (i64, i64), (i64, i64)),
) -> usize {
    let img_enhance_alg = image_input.0.clone();
    let mut input_image = image_input.1.clone();
    // let mut x_lim = image_input.2;
    // let mut y_lim = image_input.3;
    let (mut x_min, mut y_min) = image_input.2;
    let (mut y_max, mut x_max) = image_input.3;
    let mut default_c = *input_image.get(&(x_min, y_min)).unwrap();
    let mut output_image: HashMap<(i64, i64), char> = HashMap::new();
    for _ in 0..50 {
        // Apply first iteration
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                // Determine points surrounding current location
                let pos = (x, y);
                let mut surround_points: Vec<(i64, i64)> = vec![];
                for delta_y in -1..=1 {
                    for delta_x in -1..=1 {
                        surround_points.push((pos.0 + delta_x, pos.1 + delta_y));
                    }
                }
                surround_points.sort_by(|a, b| {
                    if a.1 < b.1 {
                        return Ordering::Less;
                    }
                    if a.1 == b.1 && a.0 < b.0 {
                        return Ordering::Less;
                    }
                    if a.1 == b.1 && a.0 > b.0 {
                        return Ordering::Greater;
                    }
                    if a.1 > b.1 {
                        return Ordering::Greater;
                    }
                    return Ordering::Equal;
                });
                // Determine img enhance alg index
                let mut index_string = String::new();
                for pos_s in surround_points.iter() {
                    let c = *input_image.get(&pos_s).unwrap_or(&default_c);
                    index_string.push(c);
                }
                let index = usize::from_str_radix(&index_string, 2).unwrap();
                output_image.insert((x, y), img_enhance_alg[index]);
            }
        }
        // Pad out another layer
        default_c = *output_image.get(&(x_min, y_min)).unwrap();
        x_min -= 1;
        y_min -= 1;
        x_max += 1;
        y_max += 1;
        for x_new in x_min..=x_max {
            input_image.insert((x_new, y_min), default_c);
            input_image.insert((x_new, y_max), default_c);
        }
        for y_new in y_min..=y_max {
            input_image.insert((x_min, y_new), default_c);
            input_image.insert((x_max, y_new), default_c);
        }
        // Apply next iteration
        input_image = output_image;
        output_image = HashMap::new();
    }
    return input_image.values().filter(|x| **x == '1').count();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d20_p1_actual() {
        let input = parse_input(&read_to_string("./input/2021/day20.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(5268, result);
    }

    #[test]
    fn test_d20_p2_actual() {
        let input = parse_input(&read_to_string("./input/2021/day20.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(16875, result);
    }
}
