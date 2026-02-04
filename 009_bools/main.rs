// 009. Booleans and short circuit
//
// bool is true or false. && and || stop as soon as the result is known. ! flips a bool.
// Comparisons produce bools. Use if on a bool; there is no truthiness for numbers or
// Option.
//
// Run: cargo run --bin 009_bools

fn main() {
    let ready = true;
    let empty = false;
    println!("and {} or {} not {}", ready && empty, ready || empty, !empty);

    let n = 0;
    if n != 0 && 10 / n > 1 {
        println!("skipped because n is 0");
    } else {
        println!("short circuit kept us safe");
    }
}
