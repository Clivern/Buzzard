// 153. Modules
//
// mod name; looks for name.rs or name/mod.rs next to the current file. mod name { } is
// inline. pub makes an item visible outside the module. crate:: is the crate root.
// super:: is the parent. Paths are how you keep files small.
//
// Run: cargo run --bin 153_modules

mod greet;

fn main() {
    println!("{}", greet::hello("Ada"));
}
