// 179. inspect and cloned
//
// inspect runs a side effect for each item, usually debug printing, without changing the
// pipeline. cloned clones &T into T. copied is the Copy version. flatten removes one
// level of nesting; flat_map is map then flatten.
//
// Run: cargo run --bin 179_inspect

fn main() {
    let v: Vec<_> = [1, 2, 3]
        .iter()
        .inspect(|n| println!("see {n}"))
        .copied()
        .flat_map(|n| [n, n])
        .collect();
    println!("{v:?}");
}
