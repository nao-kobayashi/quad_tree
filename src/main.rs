pub mod types;
pub mod quad_tree;

use types::Point;
use quad_tree::QuadTree;

fn main() {
    let a: f64 = 137.12345678;
    let b = a * 1000000.0;
    println!("{}", b);
    println!("{}", b.floor());
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let points = vec![
            Point::new(0, "a".to_string(), 100.0, 100.0),
            Point::new(1, "b".to_string(), 1.0, 1.0),
            Point::new(2, "c".to_string(), 50.0, 40.0),
            Point::new(3, "c".to_string(), 55.0, 45.0),
            Point::new(4, "d".to_string(), 60.0, 70.0),
        ];

        let target = Point::new(100, "target".to_string(), 61.0, 50.0);
        let mut qt = QuadTree::new(points);
        qt.init();
        let p = qt.convert_point_to_no(&target);
        assert_eq!(vec![3], qt.get_points_in_area(p.unwrap()).unwrap());

        let ans = &Point::new(3, "c".to_string(), 55.0, 45.0);
        let nearest = qt.get_nearest(&target);
        assert_eq!(Some(ans), nearest);


        let ans2 = Point::new(2, "c".to_string(), 50.0, 40.0);
        let nearest2 = qt.get_nearest(&ans2);
        assert_eq!(Some(&ans2), nearest2);
    }


    #[test]
    fn test2() {
        let points = vec![
            Point::new(0, "a".to_string(), 100.0, 100.0),
            Point::new(1, "b".to_string(), 1.0, 1.0),
            Point::new(2, "c".to_string(), 50.0, 40.0),
            Point::new(3, "c".to_string(), 51.0, 41.0),
            Point::new(4, "c".to_string(), 52.0, 43.0),
            Point::new(5, "d".to_string(), 60.0, 70.0),
            Point::new(100, "target".to_string(), 53.0, 44.0)
        ];

        let target = Point::new(4, "c".to_string(), 52.0, 42.5);
        let mut qt = QuadTree::new(points);
        qt.init();
        let p = qt.convert_point_to_no(&target);
        println!("{:?}", p);
        println!("{:?}", qt.get_area(p.unwrap()));
        let mut v =  qt.get_points_in_area(p.unwrap()).unwrap().iter().map(|n| *n).collect::<Vec<usize>>() ;
        v.sort();
        assert_eq!(vec![3, 4, 6], v);

        let ans = &Point::new(4, "c".to_string(), 52.0, 43.0);
        let target2 = &Point::new(4, "c".to_string(), 49.5, 44.0);
        let nearest = qt.get_nearest(&target2);
        assert_eq!(Some(ans), nearest);
    }

    #[test]
    fn test3() {
        let mut points = Vec::new();
        for i in 0..10000 {
            let p = Point::new(i, "test".to_string(), (i + 1) as f64, (i + 1) as f64);
            points.push(p);
        }

        let mut qt = QuadTree::new(points.clone());
        qt.init();
        for point in points {
            let p_calc = qt.convert_point_to_no(&point);
            let p_normal = qt.get_point(&point);
            if p_normal.is_some() {
                assert_eq!(p_normal, p_calc);
            } else {
                println!("{:?}", p_calc);
                println!("{:?}", point);
                println!("{:?}", qt.get_area(p_calc.unwrap()));
            }
        }
    }

    #[test]
    fn test4() {
        let mut points = Vec::new();
        for i in 0..10000 {
            let p2 = Point::new(i, "test".to_string(), (i * 100) as f64, (i * 100) as f64);
            points.push(p2);
        }

        let mut qt = QuadTree::new(points.clone());
        qt.init();
        for point in points {
            let p_calc = qt.convert_point_to_no(&point);
            let p_normal = qt.get_point(&point);
            if p_normal.is_some() {
                assert_eq!(p_normal, p_calc);
            } else {
                println!("{:?}", p_calc);
                println!("{:?}", point);
                println!("{:?}", qt.get_area(p_calc.unwrap()));
            }
        }
    }
}