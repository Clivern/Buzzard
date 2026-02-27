// 090. Extension traits
//
// You can add methods to a type you do not own by defining a trait and impling it for
// that type. This is how iterator adapters from extra crates show up as methods. The
// trait must be in scope to see the methods. You cannot impl a foreign trait for a
// foreign type (the orphan rule).
//
// Run: cargo run --bin 090_extension

trait Hex {
    fn hex(&self) -> String;
}

impl Hex for u8 {
    fn hex(&self) -> String {
        format!("{self:02x}")
    }
}

fn main() {
    println!("{}", 15u8.hex());
}
