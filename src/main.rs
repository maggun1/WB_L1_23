struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance(p1: &Point, p2: &Point) -> f64 {
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

fn main() {
    let point1 = Point::new(5.0, 6.0);
    let point2 = Point::new(2.0, 2.0);

    let dist = Point::distance(&point1, &point2);
    println!("Distance between points: {:.2}", dist);
}
