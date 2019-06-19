struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn calculate_distance(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt()
    }
}

pub fn run () {
    let p1 = Point{x:23, y:34};
    println!("{}", p1.calculate_distance());
}

