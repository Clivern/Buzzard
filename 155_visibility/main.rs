// 155. pub visibility
//
// Items are private by default. pub is crate-visible if the module path to them is pub.
// pub(crate) is the whole crate. pub(super) is the parent module. Fields of a struct are
// private unless marked pub; that is how you keep invariants.
//
// Run: cargo run --bin 155_visibility

mod inner {
    pub struct Point {
        pub x: i32,
        y: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
        pub fn y(&self) -> i32 {
            self.y
        }
    }
}

fn main() {
    let p = inner::Point::new(1, 2);
    println!("{} {}", p.x, p.y());
}
