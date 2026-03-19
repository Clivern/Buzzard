// 164. Const generics
//
// A const parameter is a value that is part of the type: struct Grid<T, const N: usize>.
// [T; N] already uses this. Functions can be generic over N so one impl covers every
// array length.
//
// Run: cargo run --bin 164_const_generics

fn first<T, const N: usize>(arr: [T; N]) -> Option<T> {
    arr.into_iter().next()
}

fn main() {
    println!("{:?} {:?}", first([1, 2, 3]), first::<i32, 0>([]));
}
