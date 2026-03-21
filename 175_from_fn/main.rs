// 175. from_fn iterators
//
// iter::from_fn builds an iterator from a closure that returns Option. iter::successors
// builds a sequence from a seed. repeat and repeat_with are infinite; pair them with
// take. These replace small custom Iterator impls.
//
// Run: cargo run --bin 175_from_fn

fn main() {
    let mut n = 0;
    let v: Vec<_> = std::iter::from_fn(|| {
        n += 1;
        if n <= 3 {
            Some(n)
        } else {
            None
        }
    })
    .collect();
    println!("{v:?}");
    let powers: Vec<_> = std::iter::successors(Some(1i32), |n| n.checked_mul(2))
        .take(6)
        .collect();
    println!("{powers:?}");
}
