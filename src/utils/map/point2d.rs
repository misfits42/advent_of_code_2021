/// Represents a single point in two-dimensional Euclidean space.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Point2D {
    x: i64,
    y: i64
}

impl Point2D {
    /// Creates a new 2D point.
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    /// Gets the value of the x-coordinate.
    pub fn get_x(&self) -> i64 {
        return self.x;
    }

    /// Updates the value of the x-coordinate.
    pub fn set_x(&mut self, x: i64) {
        self.x = x;
    }

    /// Gets the value of the y-coordinate.
    pub fn get_y(&self) -> i64 {
        return self.y;
    }

    /// Updates the value of the y-coordinate.
    pub fn set_y(&mut self, y: i64) {
        self.y = y;
    }

    /// Moves the point by the specified amount in the x- and y-directions.
    pub fn move_point(&mut self, delta_x: i64, delta_y: i64) {
        self.x += delta_x;
        self.y += delta_y;
    }

    /// Gets the eight surrounding points from the current location. Panics if integer overflow or
    /// underflow would occur.
    pub fn get_surrounding_points(&self) -> Vec<Point2D> {
        let mut output: Vec<Point2D> = vec![];
        output.push(Point2D::new(self.x, self.y - 1)); // up
        output.push(Point2D::new(self.x + 1, self.y - 1)); // diag - up right
        output.push(Point2D::new(self.x + 1, self.y)); // right
        output.push(Point2D::new(self.x + 1, self.y + 1)); // diag - down right
        output.push(Point2D::new(self.x, self.y + 1)); // down
        output.push(Point2D::new(self.x - 1, self.y + 1)); // diag - down left
        output.push(Point2D::new(self.x - 1, self.y)); // left
        output.push(Point2D::new(self.x - 1, self.y - 1)); // diag - up left
        return output;
    }

    /// Calculates the Manhattan distance between the current point and the other point.
    pub fn calculate_manhattan_distance(&self, other: &Point2D) -> u64 {
        return (self.x - other.x).abs() as u64 + (self.y - other.y).abs() as u64;
    }
}
