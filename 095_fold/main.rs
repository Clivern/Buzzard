// 095. fold and reduce
//
// fold takes an initial accumulator and a function. reduce uses the first item as the
// start, so it returns Option. sum and product are specialized folds. try_fold stops on
// an error. These are the most general consumers.
//
// Run: cargo run --bin 095_fold

fn main() {
    let sum = (1..=4).fold(0, |acc, n| acc + n);
    let prod = (1..=4).product::<i32>();
    let max = [3, 9, 1].into_iter().reduce(|a, b| a.max(b));
    println!("{sum} {prod} {max:?}");
}
