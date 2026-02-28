// 091. Iterators
//
// An iterator produces a sequence of values with next(). for in uses IntoIterator.
// iter() borrows items. iter_mut() borrows mutably. into_iter() moves items out. The
// Iterator trait has dozens of adapters built from next.
//
// Run: cargo run --bin 091_iterators

fn main() {
    let v = vec![1, 2, 3];
    let mut it = v.iter();
    println!("{:?} {:?} {:?}", it.next(), it.next(), it.next());
    println!("{:?}", it.next());
}
