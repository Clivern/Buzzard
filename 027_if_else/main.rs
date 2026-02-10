// 027. if and else
//
// if takes a bool. There are no parentheses around the condition, and the body must be a
// block. else if chains extra tests. Because if is an expression, both branches must
// have the same type when you assign the result.
//
// Run: cargo run --bin 027_if_else

fn main() {
    let n = 7;
    if n < 0 {
        println!("neg");
    } else if n == 0 {
        println!("zero");
    } else {
        println!("pos");
    }

    let parity = if n % 2 == 0 { "even" } else { "odd" };
    println!("{parity}");
}
