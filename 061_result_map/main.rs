// 061. Result combinators
//
// map transforms Ok. map_err transforms Err. and_then chains another Result. unwrap_or
// and unwrap_or_else pick a fallback. ok() turns Result into Option by dropping the
// error. These keep happy-path code linear.
//
// Run: cargo run --bin 061_result_map

fn main() {
    let r: Result<i32, &str> = Ok(3);
    println!("{:?}", r.map(|n| n + 1));
    println!("{:?}", r.map_err(|e| format!("bad:{e}")));
    println!("{}", r.unwrap_or(0));
}
