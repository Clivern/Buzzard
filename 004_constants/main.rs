// 004. Constants
//
// const names a value that is known at compile time. The type is required. The name is
// in SCREAMING_SNAKE_CASE by convention. A const can live at module scope, unlike a let
// binding which lives in a function. static is a value with a fixed address for the
// whole run; const is inlined wherever you use it.
//
// Run: cargo run --bin 004_constants

const MAX: i32 = 100;
static HOST: &str = "localhost";

fn main() {
    println!("max {MAX} host {HOST}");
}
