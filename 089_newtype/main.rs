// 089. The newtype pattern
//
// struct Watts(u32) is a new type, not an alias. You cannot pass Watts where u32 is
// required. That keeps units straight. You can impl Display, From, and Add for it. Deref
// to the inner value is optional; many newtypes expose .0 or a getter instead, so the
// wrapper stays visible.
//
// Run: cargo run --bin 089_newtype

struct Watts(u32);

impl std::fmt::Display for Watts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}W", self.0)
    }
}

fn draw(power: Watts) {
    println!("draw {power}");
}

fn main() {
    draw(Watts(60));
}
