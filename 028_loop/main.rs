// 028. loop and break values
//
// loop { } repeats until break. break can carry a value: let n = loop { break 3; }.
// continue skips the rest of this round. A loop that never breaks has type !. Prefer
// loop when the exit is in the middle, not at the top.
//
// Run: cargo run --bin 028_loop

fn main() {
    let mut n = 0;
    let found = loop {
        n += 1;
        if n == 3 {
            break n;
        }
    };
    println!("found {found}");
}
