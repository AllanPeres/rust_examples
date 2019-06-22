use std::ops::Add;

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn calculate_distance(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt()
    }
}

impl Add<Point> for Point {

    type Output = Point;

    fn add(self, point: Point) -> Self::Output {
        Point {
            x: self.x + point.x,
            y: self.y + point.y
        }
    }
}