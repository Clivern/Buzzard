// 053. Unit structs
//
// A unit struct has no fields: struct Marker;. It is useful as a type-level token, a
// trait impl with no data, or a dummy error. There is only one value, written Marker or
// Marker {}.
//
// Run: cargo run --bin 053_unit_structs

#[derive(Debug)]
struct Ready;

fn take(_: Ready) {
    println!("ready");
}

fn main() {
    take(Ready);
    println!("{Ready:?}");
}
