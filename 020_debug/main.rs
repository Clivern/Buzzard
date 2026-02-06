// 020. Debug formatting
//
// #[derive(Debug)] lets a struct print with {:?}. The Debug impl is for programmers;
// Display ({}) is for users. {:#?} indents nested values. dbg! prints file and line, and
// returns the value, so you can wrap an expression while debugging.
//
// Run: cargo run --bin 020_debug

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("{p:?}");
    println!("{p:#?}");
    let n = dbg!(1 + 2);
    println!("n {n}");
}
