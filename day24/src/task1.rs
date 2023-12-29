use std::fs;
use std::ops::{Sub};

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents = binding.lines().collect::<Vec<&str>>();

    let mut points: Vec<Entry> = Vec::new();

    for line in contents {
        points.push(Entry::new(line));
    }

    println!("{}", points.len());

    let mut count = 0;

    for i in 0..points.len() {
        for j in i+1..points.len() {
            if let Some(_) = points[i].intersection2d(&points[j]) {
                count += 1;
            }
        }
    }

    println!("{count}")
}

#[derive(Debug, Copy, Clone)]
struct Point(f64, f64, f64);

impl Point {
    fn is_parallel2d(&self, other: &Point) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(Debug)]
struct Entry {
    position: Point,
    velocity: Point
}

impl Entry {
    fn new(line: &str) -> Entry {
        let splitted = line.split(" @ ").map(|x| x.split(",").collect::<Vec<_>>()).collect::<Vec<_>>();
        let splitted: Vec<Vec<f64>> = splitted.iter().map(|x| x.iter().map(|y| y.trim().parse().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
        Entry {position: Point(splitted[0][0], splitted[0][1], splitted[0][2]), velocity: Point(splitted[1][0], splitted[1][1], splitted[1][2])}
    }

    fn intersection2d(&self, other: &Entry) -> Option<Point> {
        const BOUNDARY: (f64, f64) = (200000000000000.0, 400000000000000.0);
        /*
        (x-x_0)/s_x = (y-y_0)/s_y => x*s_y-y*s_x + y_0*s_x-x_0*s_y = 0
        
        */

        // First check if they are the same line or parallel
        if self.velocity.is_parallel2d(&other.velocity) {
            return None;
        }

        let (a0, b0, c0) = (self.velocity.1, -self.velocity.0, self.position.1 * self.velocity.0 - self.position.0 * self.velocity.1);
        let (a1, b1, c1) = (other.velocity.1, -other.velocity.0, other.position.1 * other.velocity.0 - other.position.0 * other.velocity.1);

        //println!("{a0} {b0} {c0}");
        //println!("{a1} {b1} {c1}");

        // now all the lines are in the form ax+by+c=0
        // a0x+b0y+c0=0 | * a1
        // a1x+b1y+c1=0 | * -a0

        // (a1 * b0 - a0 * b1) * y + (c0 * a1 - c1 * a0) = 0
        let intersection_y = (c1 * a0 - c0 * a1) as f64 / (a1 * b0 - a0 * b1) as f64;
        //println!("{intersection_y}");
        if intersection_y < BOUNDARY.0 || intersection_y > BOUNDARY.1 {
            return None;
        }

        let intersection_x = (-c0 as f64 - b0 as f64 * intersection_y) / a0 as f64;
        //println!("{intersection_x}");
        if intersection_x < BOUNDARY.0 || intersection_x > BOUNDARY.1 {
            return None;
        }
        
        let result = Point(intersection_x, intersection_y, 0.0);

        let time1 = result - self.position;
        let time1 = time1.0 / self.velocity.0;

        let time2 = result - other.position;
        let time2 = time2.0 / other.velocity.0;

        //println!("{time1} {time2}");

        if time1 < 0.0 || time2 < 0.0 {
            return None;
        }

        Some(result)
    }
}