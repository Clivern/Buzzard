// 055. Associated functions
//
// Functions in impl with no self are associated functions. Counter::new() is the usual
// constructor. They are namespaced by the type, unlike a free function. Self in impl is
// an alias for the type being implemented.
//
// Run: cargo run --bin 055_associated

struct Counter {
    n: i32,
}

impl Counter {
    fn new() -> Self {
        Self { n: 0 }
    }
}

fn main() {
    let c = Counter::new();
    println!("{}", c.n);
}
