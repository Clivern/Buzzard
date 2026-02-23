// 072. Hash
//
// Hash feeds bytes into a hasher. HashMap requires Eq + Hash on keys. Derive Hash with
// Eq. Do not hash only a prefix of a key if == uses the whole key; the contract is k1 ==
// k2 implies hash(k1) == hash(k2).
//
// Run: cargo run --bin 072_hash

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Debug)]
struct User<'a> {
    id: u64,
    name: &'a str,
}

impl Hash for User<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn main() {
    let mut m = HashMap::new();
    m.insert(User { id: 1, name: "ada" }, "admin");
    println!("{:?}", m.get(&User { id: 1, name: "ignored" }));
}
