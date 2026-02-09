// 022. Parameters and patterns
//
// Parameters can destructure. fn head((x, _): (i32, i32)) binds x and ignores the rest.
// mut on a parameter lets you assign to that local copy; it does not mutate the caller.
// References in parameters borrow instead of taking ownership.
//
// Run: cargo run --bin 022_params

fn head((x, _): (i32, i32)) -> i32 {
    x
}

fn bump_copy(mut n: i32) -> i32 {
    n += 1;
    n
}

fn main() {
    println!("{}", head((9, 1)));
    let n = 4;
    println!("copy {} original {n}", bump_copy(n));
}
