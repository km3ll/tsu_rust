#![allow(unused)]
use c7_orphan_rule::Point;

// Error: only traits defined in the current crate can be implemented for traits
// defined ouside of the trait. We are breaking the orphan rule.
// impl PartialEq for Point {

// The Orphan Rule
// In order to implement a trait on a given type, either the trait or the type must
// be defined within the current crate.

// There is a way to get around this rule by creating a Wrapper Type
// PointWrapper is a tuple struct
struct PointWrapper(Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

fn main() {

    let p1 = PointWrapper(Point { x: 1, y: 2 });
    let p2 = PointWrapper(Point { x: 1, y: 2 });
    println!("p1 == p2: {}", p1 == p2)

}
