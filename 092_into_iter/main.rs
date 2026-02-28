// 092. iter vs into_iter
//
// v.iter() yields &T. v.iter_mut() yields &mut T. v.into_iter() yields T and consumes v.
// for x in &v is iter. for x in &mut v is iter_mut. for x in v is into_iter. On arrays,
// into_iter also moves by value as of recent Rust.
//
// Run: cargo run --bin 092_into_iter

fn main() {
    let mut v = vec![1, 2, 3];
    for n in &v {
        println!("borrow {n}");
    }
    for n in &mut v {
        *n += 10;
    }
    for n in v {
        println!("owned {n}");
    }
}
