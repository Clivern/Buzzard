// 037. if let
//
// if let pattern = expr is a one-arm match. Use it when you only care about one variant
// and want to ignore the rest. else handles the other cases. Prefer match when there are
// several meaningful variants.
//
// Run: cargo run --bin 037_if_let

fn main() {
    let maybe = Some("hi");
    if let Some(msg) = maybe {
        println!("{msg}");
    } else {
        println!("nothing");
    }
}
