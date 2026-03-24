// 190. Raw pointers
//
// *const T and *mut T are raw pointers. They can be null, dangling, or unaligned.
// Creating them is safe; dereferencing is not. as_ptr() on a slice gives a pointer to
// the first element. Offset is in units of T, not bytes. Prefer references and slices
// unless you talk to C or build a data structure.
//
// Run: cargo run --bin 190_raw_pointers

fn main() {
    let mut a = [1, 2, 3];
    let p = a.as_mut_ptr();
    unsafe {
        *p.add(1) = 9;
    }
    println!("{a:?}");
    let n: *const i32 = std::ptr::null();
    println!("null {}", n.is_null());
}
