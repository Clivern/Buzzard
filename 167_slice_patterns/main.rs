// 167. Slice patterns
//
// You can match on slices: [first, rest @ ..], [a, b], []. This works on arrays and on
// &[T]. rest @ .. binds a subslice. It is exhaustive only if you cover every length you
// care about, usually with a last _ arm.
//
// Run: cargo run --bin 167_slice_patterns

fn main() {
    let v = [1, 2, 3];
    match v {
        [first, rest @ ..] => println!("{first} {rest:?}"),
    }
    match &v[..] {
        [] => println!("empty"),
        [x] => println!("one {x}"),
        [a, b, ..] => println!("at least two {a} {b}"),
    }
}
