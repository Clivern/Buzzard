// 033. Labeled loops
//
// 'outer: loop names a loop so break and continue can talk to it from an inner loop.
// break 'outer leaves the outer loop. continue 'outer starts the next outer round.
// Labels start with a quote.
//
// Run: cargo run --bin 033_labels

fn main() {
    let mut hits = 0;
    'outer: for i in 0..4 {
        for j in 0..4 {
            if i == 2 && j == 1 {
                break 'outer;
            }
            hits += 1;
        }
    }
    println!("hits {hits}");
}
