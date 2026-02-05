// 012. Arrays
//
// An array has a fixed length that is part of the type: [i32; 3]. All elements share one
// type. Indexing is a[0]. a.len() is the length. Arrays live on the stack. A slice &[T]
// is a view into an array or a Vec. Out-of-range indexing panics.
//
// Run: cargo run --bin 012_arrays

fn main() {
    let a: [i32; 3] = [1, 2, 3];
    let zeros = [0; 4];
    println!("first {} len {} zeros {:?}", a[0], a.len(), zeros);
    let view: &[i32] = &a[1..];
    println!("tail {view:?}");
}
