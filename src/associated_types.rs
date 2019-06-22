use std::ops::Add;

use super::objects::point::Point;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_points() {
        let p1 = Point{ x: 2, y: 3};
        let p2 = Point{ x: 5, y: -1};
        let p3 = p1 + p2;
        assert_eq!(p3.x, 7);
        assert_eq!(p3.y, 2);
    }

    #[test]
    fn test_calculate_distances() {
        let p1 = Point{ x: 1, y: 3 };
        assert_eq!(p1.calculate_distance(), (10 as f64).sqrt());
    }
}