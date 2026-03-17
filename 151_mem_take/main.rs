// 151. mem::take
//
// take(v) is replace(v, Default::default()). It is the usual way to move out of a
// mutable reference when the type implements Default: you leave an empty Vec or String
// behind so the owner can still drop it.
//
// Run: cargo run --bin 151_mem_take

fn main() {
    let mut v = vec![1, 2, 3];
    let taken = std::mem::take(&mut v);
    println!("{taken:?} left {v:?}");
}
