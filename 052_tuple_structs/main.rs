// 052. Tuple structs
//
// A tuple struct has fields without names: struct Color(u8, u8, u8). It is a distinct
// type, unlike a bare tuple. Access fields with .0, .1. Newtypes such as struct
// UserId(u64) wrap one value so you cannot pass a raw u64 by mistake.
//
// Run: cargo run --bin 052_tuple_structs

struct Color(u8, u8, u8);
struct UserId(u64);

fn main() {
    let red = Color(255, 0, 0);
    let id = UserId(42);
    println!("r {} id {}", red.0, id.0);
}
