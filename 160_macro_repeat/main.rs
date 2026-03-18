// 160. Macro repetition
//
// $($item:expr),* matches a comma-separated list. You can expand the repetition in the
// body. A second matcher can provide a different arity. Keep macros small enough to read
// the expansion in rust-analyzer.
//
// Run: cargo run --bin 160_macro_repeat

macro_rules! vec_of {
    ($($x:expr),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push($x);)*
            v
        }
    };
}

fn main() {
    println!("{:?}", vec_of![1, 2, 3]);
}
