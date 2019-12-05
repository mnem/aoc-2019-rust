use common::Puzzle;
use std::str::FromStr;
use std::ops;
use std::collections::HashSet;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();
}

#[derive(Default)]
struct Puzzle1 {
    a: Option<Wire>,
    b: Option<Wire>,
}

impl Puzzle for Puzzle1 {
    type ParsedLine = Wire;

    fn process_item(&mut self, item: Self::ParsedLine) {
        if self.a.is_none() {
            self.a = Some(item);
        } else if self.b.is_none() {
            self.b = Some(item);
        } else {
            panic!("Too many wires!");
        }
    }

    fn final_result(&mut self) -> String {
        let cross = self.a.as_ref().unwrap().closest_cross(self.b.as_ref().unwrap()).manhatten_from_zero().to_string();
        let steps = self.a.as_ref().unwrap().shortest_intersection_combined_distance(self.b.as_ref().unwrap());
        return format!("cross: {}, steps: {}", cross, steps);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64
}

const ZERO_POINT: Point = Point { x: 0, y: 0 };

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Point {
    fn manhatten_from_zero(&self) -> i64 {
        self.manhatten( &ZERO_POINT)
    }

    fn manhatten(&self, origin: &Point) -> i64 {
        (self.x - origin.x).abs() + (self.y - origin.y).abs()
    }
}

struct Wire {
    points: Vec<Point>
}

impl FromStr for Wire {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let movements: Vec<&str> = s.split(",")
            .map(|s| s.trim())
            .collect();

        let mut points = Vec::new();
        let mut current = Point { x: 0, y: 0 };
        for movement in movements {
            let direction = movement.chars().nth(0).unwrap();
            let count:i64 = movement[1..].parse().unwrap();

            let advance_by = match direction {
                'U' => Point { x: 0, y: 1 },
                'D' => Point { x: 0, y: -1 },
                'R' => Point { x: 1, y: 0 },
                'L' => Point { x: -1, y: 0 },
                _ => panic!("Hmm... I don't know that direction"),
            };

            for _ in 0 .. count {
                current = current + advance_by;
                points.push(current);
            }

        }

        Ok( Wire { points: points } )
    }
}

impl Wire {
    fn intersections<'a>(&'a self, other: &'a Wire) -> HashSet<&'a Point> {
        let a: HashSet<&Point> = self.points.iter().collect();
        let b: HashSet<&Point> = other.points.iter().collect();
        a.intersection(&b).copied().collect()
    }

    fn closest_cross(&self, other: &Wire) -> Point {
        let intersections = self.intersections(other);
        intersections
            .into_iter()
            .min_by(|a, b| a.manhatten_from_zero().cmp(&b.manhatten_from_zero()))
            .unwrap()
            .clone()
    }

    fn steps_to_point(&self, target: &Point) -> Option<i64> {
        let mut steps = 0;
        for point in self.points.iter() {
            steps += 1;
            if point == target {
                return Some(steps);
            }
        }
        return None;
    }

    fn shortest_intersection_combined_distance(&self, other: &Wire) -> i64 {
        let mut shortest = std::i64::MAX;
        for point in self.intersections(other) {
            let steps = self.steps_to_point(point).unwrap();
            let steps_other = other.steps_to_point(point).unwrap();
            shortest = shortest.min(steps + steps_other);
        }
        return shortest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_example_parsing() {
        let input_a = "R8,U5,L5,D3";
        let input_b = "U7,R6,D4,L4";

        let wire_a: Wire = input_a.parse().unwrap();
        let wire_b: Wire = input_b.parse().unwrap();

        assert_eq!(wire_a.points.len(), 21);
        assert_eq!(wire_b.points.len(), 21);

        let closest = wire_a.closest_cross(&wire_b);
        assert_eq!(closest.manhatten_from_zero(), 6 );
    }

    #[test]
    fn test_second_example_parsing() {
        let input_a = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let input_b = "U62,R66,U55,R34,D71,R55,D58,R83";

        let wire_a: Wire = input_a.parse().unwrap();
        let wire_b: Wire = input_b.parse().unwrap();

        let closest = wire_a.closest_cross(&wire_b);
        assert_eq!(closest.manhatten_from_zero(), 159 );
    }

    #[test]
    fn test_third_example_parsing() {
        let input_a = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let input_b = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        let wire_a: Wire = input_a.parse().unwrap();
        let wire_b: Wire = input_b.parse().unwrap();

        let closest = wire_a.closest_cross(&wire_b);
        assert_eq!(closest.manhatten_from_zero(), 135 );
    }

    #[test]
    fn test_first_example_steps() {
        let input_a = "R8,U5,L5,D3";
        let input_b = "U7,R6,D4,L4";

        let wire_a: Wire = input_a.parse().unwrap();
        let wire_b: Wire = input_b.parse().unwrap();

        let shortest = wire_a.shortest_intersection_combined_distance(&wire_b);
        assert_eq!(shortest, 30 );
    }

    #[test]
    fn test_second_example_steps() {
        let input_a = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let input_b = "U62,R66,U55,R34,D71,R55,D58,R83";

        let wire_a: Wire = input_a.parse().unwrap();
        let wire_b: Wire = input_b.parse().unwrap();

        let shortest = wire_a.shortest_intersection_combined_distance(&wire_b);
        assert_eq!(shortest, 610 );
    }

    #[test]
    fn test_third_example_steps() {
        let input_a = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let input_b = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        let wire_a: Wire = input_a.parse().unwrap();
        let wire_b: Wire = input_b.parse().unwrap();

        let shortest = wire_a.shortest_intersection_combined_distance(&wire_b);
        assert_eq!(shortest, 410 );
    }
}
