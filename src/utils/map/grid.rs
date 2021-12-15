pub fn get_surrounding_points_no_diagonals(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    // Top left
    if x == 0 && y == 0 {
        vec![(x, y + 1), (x + 1, y)]
    // Bottom left
    } else if x == 0 && y == y_max {
        vec![(x, y - 1), (x + 1, y)]
    // Mid left
    } else if x == 0 && y < y_max {
        vec![
            (x, y - 1),
            (x + 1, y),
            (x, y + 1),
        ]
    // Top right
    } else if x == x_max && y == 0 {
        vec![(x - 1, y), (x, y + 1)]
    // Top mid
    } else if x < x_max && y == 0 {
        vec![
            (x - 1, y),
            (x, y + 1),
            (x + 1, y),
        ]
    // Bottom right
    } else if x == x_max && y == y_max {
        vec![(x - 1, y), (x, y - 1)]
    // Bottom mid
    } else if x < x_max && y == y_max {
        vec![
            (x - 1, y),
            (x, y - 1),
            (x + 1, y),
        ]
    // Mid right
    } else if x == x_max && y < y_max {
        vec![
            (x, y + 1),
            (x - 1, y),
            (x, y - 1),
        ]
    } else {
        vec![
            (x, y - 1),
            (x + 1, y),
            (x, y + 1),
            (x - 1, y),
        ]
    }
}

/// Calculates the points around (x, y) in a 2D grid with non-negative indices bounded by given
/// maximum x- and y-values.
pub fn get_surrounding_points(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    // Top left
    if x == 0 && y == 0 {
        vec![(x, y + 1), (x + 1, y + 1), (x + 1, y)]
    // Bottom left
    } else if x == 0 && y == y_max {
        vec![(x, y - 1), (x + 1, y - 1), (x + 1, y)]
    // Mid left
    } else if x == 0 && y < y_max {
        vec![
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
        ]
    // Top right
    } else if x == x_max && y == 0 {
        vec![(x - 1, y), (x - 1, y + 1), (x, y + 1)]
    // Top mid
    } else if x < x_max && y == 0 {
        vec![
            (x - 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
            (x + 1, y),
        ]
    // Bottom right
    } else if x == x_max && y == y_max {
        vec![(x - 1, y), (x - 1, y - 1), (x, y - 1)]
    // Bottom mid
    } else if x < x_max && y == y_max {
        vec![
            (x - 1, y),
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
        ]
    // Mid right
    } else if x == x_max && y < y_max {
        vec![
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
            (x, y - 1),
        ]
    } else {
        vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
        ]
    }
}
