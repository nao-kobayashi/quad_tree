pub mod types;
pub mod quad_tree;

use types::Point;
use quad_tree::QuadTree;

fn main() {
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let points = vec![
            Point::new(0, "a".to_string(), 100, 100),
            Point::new(1, "b".to_string(), 1, 1),
            Point::new(2, "c".to_string(), 50, 40),
            Point::new(3, "c".to_string(), 55, 45),
            Point::new(4, "d".to_string(), 60, 70),
        ];

        let target = Point::new(100, "target".to_string(), 61, 50);
        let qt = QuadTree::new(points);
        let p = qt.convert_point_to_no(&target);
        println!("{:?}", p);
        let area = qt.get_area(p.unwrap());
        assert_eq!(vec![2, 3], area.is_in_points);

        let ans = &Point::new(3, "c".to_string(), 55, 45);
        let nearest = qt.get_nearest(&target);
        assert_eq!(Some(ans), nearest);
    }


    #[test]
    fn test2() {
        let points = vec![
            Point::new(0, "a".to_string(), 100, 100),
            Point::new(1, "b".to_string(), 1, 1),
            Point::new(2, "c".to_string(), 50, 40),
            Point::new(4, "d".to_string(), 60, 70),
        ];

        let target = Point::new(100, "target".to_string(), 61, 50);
        let qt = QuadTree::new(points);
        let p = qt.convert_point_to_no(&target);
        let area = qt.get_area(p.unwrap());
        assert_eq!(vec![2], area.is_in_points);

        let ans = &Point::new(2, "c".to_string(), 50, 40);
        let nearest = qt.get_nearest(&target);
        assert_eq!(Some(ans), nearest);
    }

    #[test]
    fn test3() {
        let mut points = Vec::new();
        for i in 0..10000 {
            let p = Point::new(i, "test".to_string(), (i + 1) as i32, (i + 1) as i32);
            points.push(p);
        }

        let qt = QuadTree::new(points.clone());
        for point in points {
            let p_calc = qt.convert_point_to_no(&point);
            let p_normal = qt.get_point(&point);
            if p_normal.is_some() {
                assert_eq!(p_normal, p_calc);
            }
        }
    }

    #[test]
    fn test4() {
        let mut points = Vec::new();
        for i in 0..10000 {
            let p2 = Point::new(i, "test".to_string(), (i * 100) as i32, (i * 100) as i32);
            points.push(p2);
        }

        let qt = QuadTree::new(points.clone());
        for point in points {
            let p_calc = qt.convert_point_to_no(&point);
            let p_normal = qt.get_point(&point);
            if p_normal.is_some() {
                assert_eq!(p_normal, p_calc);
            }
        }
    }
}