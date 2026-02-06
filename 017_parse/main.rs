// 017. Parsing from strings
//
// str::parse uses the FromStr trait. It returns Result, because the text might not be a
// valid value. unwrap is fine in tiny examples; real code uses ? or match. You pick the
// output type with a type annotation or a turbofish.
//
// Run: cargo run --bin 017_parse

fn main() {
    let ok: Result<i32, _> = "42".parse();
    let bad: Result<i32, _> = "xi".parse();
    println!("{ok:?} {bad:?}");
    match "2.5".parse::<f64>() {
        Ok(v) => println!("float {v}"),
        Err(e) => println!("err {e}"),
    }
}
