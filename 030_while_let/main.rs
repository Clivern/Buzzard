// 030. while let
//
// while let pattern = expr repeats while the pattern matches. It is the loop form of if
// let. A common use is popping a stack until it is empty, or reading iterator next()
// until None.
//
// Run: cargo run --bin 030_while_let

fn main() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("pop {top}");
    }
}
