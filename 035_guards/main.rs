// 035. Match guards
//
// if after a pattern is a match guard. The arm runs only when the pattern matches and
// the guard is true. Guards can look at the bound names. You still need a later arm that
// covers the rest, because the guard might fail.
//
// Run: cargo run --bin 035_guards

fn main() {
    let n = 8;
    match n {
        x if x < 0 => println!("neg"),
        x if x % 2 == 0 => println!("even {x}"),
        x => println!("odd {x}"),
    }
}
