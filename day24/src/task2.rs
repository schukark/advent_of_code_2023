use std::fs;
use std::ops::{Sub, SubAssign, Add};

const eps: f64 = 1e-3;

pub fn execute() {
    let binding = fs::read_to_string("txts/sample.txt").expect("Can't open file");
    let contents = binding.lines().collect::<Vec<&str>>();

    let mut points: Vec<Point> = Vec::new();

    for line in contents {
        points.push(Point::new(line));
    }

    for start_point in &points {
        let mut new_points = Vec::clone(&points);
        // shift to (0, 0, 0) and make it immovable
        for point in &mut new_points {
            *point -= *start_point;
        }

        // Select two points to make a line
        let zero = Point::new("0, 0, 0 @ 0, 0, 0");
        let (mut point1, mut point2) = (&zero.clone(), &zero.clone());
        for point in &new_points {
            if point == start_point {
                continue;
            }
            if *point1 == zero {
                point1 = point;
            }
            else if *point2 == zero {
                point2 = point;
            }
            else {
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector(f64, f64, f64);

impl Vector {
    fn new(coords: &Vec<f64>) -> Vector {
        assert_eq!(coords.len(), 3);
        Vector(coords[0], coords[1], coords[2])
    }

    fn cross_prod(&self, other: &Vector) -> Vector {
        let new_x = self.1 * other.2 - self.2 * other.1;
        let new_y = self.2 * other.0 - self.0 * other.2;
        let new_z = self.0 * other.1 - self.1 * other.0;

        Vector(new_x, new_y, new_z)
    }

    fn dot_prod(&self, other: &Vector) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn mul_constant(&self, constant: f64) -> Vector {
        Vector(self.0 * constant, self.1 * constant, self.2 * constant)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (f64::abs(self.0 - other.0) < eps) && (f64::abs(self.1 - other.1) < eps) && (f64::abs(self.2 - other.2) < eps)
    }
}

impl Eq for Vector {}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector{
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector{
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    position: Vector,
    velocity: Vector,
}

impl Point {
    fn new(line: &str) -> Point {
        let splitted = line.split(" @ ").collect::<Vec<_>>();
        let splitted = splitted.iter().map(|x| x.split(",").map(|y| y.trim().parse::<f64>().unwrap()).collect::<Vec<f64>>()).collect::<Vec<_>>();
        Point {position: Vector::new(&splitted[0]), velocity: Vector::new(&splitted[1])}
    }
    
    fn intersects(&self, other: &Point) -> bool {
        // p1 + c*v1 and p2 + d*v2 intersect if and only if (v1 \times v2) \cdot (p1 - p2) == 0
        f64::abs(self.velocity.cross_prod(&other.velocity).dot_prod(&(self.position - other.position))) < eps
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {position: self.position - other.position, velocity: self.velocity - other.velocity}
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            position: self.position - other.position,
            velocity: self.velocity - other.velocity,
        };
    }
}