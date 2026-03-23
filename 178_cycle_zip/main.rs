// 178. cycle and zip
//
// cycle repeats an iterator forever; the items must be Clone. zip pairs two iterators.
// unzip splits a pair iterator back. enumerate is zip with a count.
//
// Run: cargo run --bin 178_cycle_zip

fn main() {
    let names = ["a", "b"];
    let nums = 1..;
    let pairs: Vec<_> = names.into_iter().cycle().zip(nums).take(5).collect();
    println!("{pairs:?}");
}
