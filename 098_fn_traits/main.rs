// 098. Fn, FnMut, FnOnce
//
// FnOnce can be called once (it may consume captured values). FnMut can be called many
// times and may mutate captures. Fn can be called many times without mutating. Closures
// implement the most they can. fn pointers implement all three. A parameter impl Fn(i32)
// -> i32 accepts Fn closures and function items.
//
// Run: cargo run --bin 098_fn_traits

fn apply<F>(n: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(n)
}

fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("{} {}", apply(3, |n| n + 1), apply(3, double));
}
