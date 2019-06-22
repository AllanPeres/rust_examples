use super::objects::point::Point;

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

trait InvertedPoint<T> {
    fn invert(&self) -> T;
}

impl InvertedPoint<Point> for Point {
    fn invert(&self) -> Point {
        Point {
            x: self.y,
            y: self.x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(max('a', 'b'), 'b');
    }

    #[test]
    fn test_inverted_point() {
        let p1 = Point{ x: 1, y: 2};
        let p2 = p1.invert();
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 1);
        assert_eq!(p1.calculate_distance(), p2.calculate_distance());
    }
}