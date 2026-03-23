// 177. Peekable
//
// peekable() lets you look at next without consuming it. peek() returns Option<&Item>.
// Useful in parsers: decide whether to take a token. The peeked value stays until
// next().
//
// Run: cargo run --bin 177_peekable

fn main() {
    let mut it = [1, 2, 3].into_iter().peekable();
    println!("{:?}", it.peek());
    println!("{:?}", it.next());
    println!("{:?}", it.peek());
}
