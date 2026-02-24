// 076. Borrow and ToOwned
//
// Borrow<T> is like AsRef but promises Hash/Eq/Ord match the owned type, so a
// HashMap<String, V> can be queried with &str. ToOwned builds an owned value from a
// borrow: str::to_owned() is a String. clone on a &T also works when T: Clone.
//
// Run: cargo run --bin 076_borrow_trait

use std::collections::HashMap;

fn main() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert(String::from("a"), 1);
    let k: &str = "a";
    println!("{:?}", m.get(k));
    println!("{:?}", m.get("a"));
    println!("{}", "hi".to_owned());
}
