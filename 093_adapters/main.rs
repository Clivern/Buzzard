// 093. Iterator adapters
//
// map, filter, take, skip, enumerate, zip, flatten, inspect are lazy: they do nothing
// until you consume the iterator. collect, for, count, sum, fold consume it. Chain
// adapters to describe the work, then collect once.
//
//
// inspect is lazy too: it only prints when a later consumer pulls an item. That is why
// adapters can be stacked cheaply.
//
// Run: cargo run --bin 093_adapters

fn main() {
    let v: Vec<_> = (0..10)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .take(3)
        .collect();
    println!("{v:?}");
    let n: i32 = (1..5).filter(|n| *n > 1).sum();
    println!("sum {n}");
}
