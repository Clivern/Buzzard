// 079. Generic structs
//
// A struct can have type parameters. Point<T> { x: T, y: T } works for i32 and f64. You
// can impl Point<i32> only, or impl<T> Point<T> for all T. Multiple parameters Point<T,
// U> allow mixed coordinates.
//
// Run: cargo run --bin 079_generic_structs

struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> Point<T> {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

fn main() {
    Point { x: 1, y: 2 }.show();
    Point { x: 1.5, y: 2.5 }.show();
}
