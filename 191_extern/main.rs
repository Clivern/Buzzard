// 191. extern and FFI
//
// extern "C" fn is the C calling convention. #[no_mangle] keeps the name for linking.
// libc functions are declared this way. The unsafe is at the call: you promise the
// signature and the preconditions match. This lesson only calls strlen on a C string we
// built with CString.
//
// Run: cargo run --bin 191_extern

use std::ffi::CString;

unsafe extern "C" {
    fn strlen(s: *const i8) -> usize;
}

fn main() {
    let s = CString::new("hello").unwrap();
    let n = unsafe { strlen(s.as_ptr()) };
    println!("{n}");
}
