// 066. From and Into
//
// From<T> for U builds a U from a T. Into is the reverse, and is implemented
// automatically if From exists. .into() needs a type annotation when inference cannot
// tell the target. Prefer implementing From, not Into.
//
// Run: cargo run --bin 066_from_into

struct Id(u32);

impl From<u32> for Id {
    fn from(n: u32) -> Self {
        Id(n)
    }
}

fn main() {
    let a = Id::from(7);
    let b: Id = 8.into();
    println!("{} {}", a.0, b.0);
}
