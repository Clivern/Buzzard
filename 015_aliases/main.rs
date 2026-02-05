// 015. Type aliases
//
// type names an existing type. It does not create a new type, so Kilometers and i32 are
// interchangeable. Use a newtype struct when you want the compiler to reject mixing two
// meanings of the same underlying type.
//
// Run: cargo run --bin 015_aliases

type Kilometers = i32;

fn fly(distance: Kilometers) {
    println!("flew {distance} km");
}

fn main() {
    let d: Kilometers = 40;
    let raw: i32 = d;
    fly(raw);
}
