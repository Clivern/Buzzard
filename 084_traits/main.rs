// 084. Traits
//
// A trait is a set of methods a type can implement. There is no implements keyword: impl
// Trait for Type. A trait can provide default method bodies. You can use trait methods
// only if the trait is in scope.
//
// Run: cargo run --bin 084_traits

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

fn main() {
    println!("{}", Circle { r: 2.0 }.area());
}
