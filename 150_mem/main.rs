// 150. std::mem
//
// size_of::<T>() is the size in bytes. size_of_val looks at a value (useful for slices).
// swap exchanges two values. take replaces with Default and returns the old. replace
// puts a new value in and returns the old. drop is also here.
//
// Run: cargo run --bin 150_mem

fn main() {
    println!("i32 {}", std::mem::size_of::<i32>());
    let mut a = 1;
    let mut b = 2;
    std::mem::swap(&mut a, &mut b);
    println!("{a} {b}");
    let old = std::mem::replace(&mut a, 9);
    println!("old {old} now {a}");
}
