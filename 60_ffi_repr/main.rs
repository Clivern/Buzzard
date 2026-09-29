// 60. FFI and repr: calling C, exposing Rust to C, and memory layout
//
// An extern "C" block declares functions that live in another library and use the C
// calling convention. In edition 2024 the block is written unsafe extern "C", and each
// call is unsafe: you are promising the signature matches and the preconditions hold.
// libc is linked by default, so functions such as strlen, abs, and qsort are available
// here.
//
// Strings cross the boundary as NUL-terminated bytes. CString owns one built from Rust
// data; CStr borrows one handed over by C. A Rust function marked extern "C" can be
// called from C, or passed to C as a callback; #[unsafe(no_mangle)] keeps its symbol
// name for linking.
//
// The default Rust layout is unspecified and may reorder fields. #[repr(C)] lays a
// struct out like C would, which FFI needs. #[repr(u8)] (or another integer) fixes an
// enum's discriminant type, and #[repr(transparent)] guarantees a newtype has exactly
// its inner type's layout.
//
// Run: cargo run --bin 60_ffi_repr

use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::mem::offset_of;

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn abs(n: c_int) -> c_int;
    fn qsort(
        base: *mut c_void,
        n: usize,
        size: usize,
        cmp: extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
}

// Example 1: calling C functions.
fn demo_call_c() {
    println!("-- calling C --");

    let s = CString::new("hello").unwrap();
    let n = unsafe {
        // SAFETY: s is a valid NUL-terminated string that outlives the call.
        strlen(s.as_ptr())
    };

    let a = unsafe { abs(-42) };
    println!("strlen {n} abs {a}");

    // A Rust &str is not NUL-terminated, so it must not be passed to strlen directly.
}

// Example 2: CString and CStr.
fn demo_c_strings() {
    println!("-- CString / CStr --");

    let owned = CString::new("ffi text").unwrap();
    println!("bytes with nul {:?}", owned.as_bytes_with_nul());
    println!("interior nul is rejected: {}", CString::new("a\0b").is_err());

    let lit: &CStr = c"a C string literal";
    println!("{:?} len {}", lit.to_str(), lit.count_bytes());

    let borrowed = unsafe {
        // SAFETY: the pointer comes from a live CString, so it is valid and NUL-terminated.
        CStr::from_ptr(owned.as_ptr())
    };

    println!("round trip {}", borrowed.to_string_lossy());
}

extern "C" fn compare_i32(a: *const c_void, b: *const c_void) -> c_int {
    // SAFETY: qsort only passes pointers to elements of the i32 array we gave it.
    let (a, b) = unsafe { (*(a as *const i32), *(b as *const i32)) };
    a.cmp(&b) as c_int
}

// Example 3: passing a Rust callback to C.
fn demo_callback() {
    println!("-- callbacks --");

    let mut v = [5, 3, 9, 1, 7];
    unsafe {
        // SAFETY: base, count, and element size describe v exactly, and the comparator
        // matches qsort's expected signature.
        qsort(v.as_mut_ptr() as *mut c_void, v.len(), size_of::<i32>(), compare_i32);
    }

    println!("sorted by libc qsort: {v:?}");
}

/// Exported with a stable C name. A C program linking this binary's code could call it.
#[unsafe(no_mangle)]
pub extern "C" fn buzzard_add(a: c_int, b: c_int) -> c_int {
    a + b
}

// Example 4: Rust functions with the C ABI.
fn demo_export() {
    println!("-- exporting --");

    println!("buzzard_add(2, 3) = {}", buzzard_add(2, 3));
    let f: extern "C" fn(c_int, c_int) -> c_int = buzzard_add;
    println!("as a C function pointer: {}", f(10, 20));
}

#[allow(dead_code)]
struct RustLayout {
    a: u8,
    b: u32,
    c: u8,
}

#[repr(C)]
struct CLayout {
    a: u8,
    b: u32,
    c: u8,
}

// Example 5: repr(C) and field offsets.
fn demo_repr_c() {
    println!("-- repr(C) --");

    println!(
        "Rust layout {} bytes, C layout {} bytes",
        size_of::<RustLayout>(),
        size_of::<CLayout>()
    );
    println!(
        "C offsets a={} b={} c={}",
        offset_of!(CLayout, a),
        offset_of!(CLayout, b),
        offset_of!(CLayout, c)
    );

    let v = CLayout { a: 1, b: 2, c: 3 };
    println!("values {} {} {}", v.a, v.b, v.c);
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tag {
    A = 1,
    B = 2,
    C = 200,
}

impl TryFrom<u8> for Tag {
    type Error = u8;
    fn try_from(n: u8) -> Result<Self, u8> {
        match n {
            1 => Ok(Tag::A),
            2 => Ok(Tag::B),
            200 => Ok(Tag::C),
            other => Err(other),
        }
    }
}

// Example 6: integer-backed enums.
fn demo_repr_enum() {
    println!("-- repr(u8) enum --");

    println!(
        "{} {} {} size {}",
        Tag::A as u8,
        Tag::B as u8,
        Tag::C as u8,
        size_of::<Tag>()
    );
    println!("from wire {:?} {:?}", Tag::try_from(2), Tag::try_from(9));

    // Casting an arbitrary integer into the enum with transmute would be undefined
    // behaviour for 9; checked conversion is the safe path.
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct Handle(u64);

// Example 7: repr(transparent) newtypes.
fn demo_transparent() {
    println!("-- repr(transparent) --");

    println!(
        "Handle {} bytes, align {}, same as u64: {}",
        size_of::<Handle>(),
        align_of::<Handle>(),
        size_of::<Handle>() == size_of::<u64>()
    );

    let raw: [u64; 2] = [7, 8];
    let handles: &[Handle] = unsafe {
        // SAFETY: Handle is repr(transparent) over u64, so the layouts are identical.
        std::slice::from_raw_parts(raw.as_ptr() as *const Handle, raw.len())
    };

    println!("reinterpreted {handles:?}");
}

fn main() {
    demo_call_c();
    demo_c_strings();
    demo_callback();
    demo_export();
    demo_repr_c();
    demo_repr_enum();
    demo_transparent();
}
