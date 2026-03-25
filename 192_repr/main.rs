// 192. repr
//
// #[repr(C)] lays out a struct like C, for FFI. #[repr(u8)] on an enum sets the
// discriminant size. #[repr(transparent)] is for newtypes that must match the inner
// type's layout. The default Rust layout is unspecified and may reorder fields.
//
// Run: cargo run --bin 192_repr

#[repr(C)]
struct Pair {
    a: u8,
    b: u32,
}

#[repr(u8)]
enum Tag {
    A = 1,
    B = 2,
}

fn main() {
    println!("{} {} {}", std::mem::size_of::<Pair>(), Tag::A as u8, Tag::B as u8);
}
