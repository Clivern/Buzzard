// 59. unsafe, raw pointers, and MaybeUninit
//
// unsafe unlocks five things the compiler cannot check for you: dereferencing a raw
// pointer, calling an unsafe fn, reading or writing a static mut, implementing an
// unsafe trait, and accessing union fields. It does not turn off the borrow checker for
// ordinary references. Keep unsafe blocks small, and write a SAFETY comment saying why
// each one is sound.
//
// *const T and *mut T are raw pointers. They may be null, dangling, or unaligned, and
// they carry no lifetime. Creating them is safe; reading or writing through them is not.
// add and offset move in units of T, not bytes.
//
// MaybeUninit<T> is memory that may not hold a valid T yet. Write to it first, then
// call the unsafe assume_init. The usual goal of unsafe code is to build a safe API on
// top, like the split_at_mut and Vec-like examples below.
//
// Run: cargo run --bin 59_unsafe

use std::mem::MaybeUninit;
use std::ptr;

// Example 1: raw pointers from references.
fn demo_raw_pointers() {
    println!("-- raw pointers --");

    let n = 5;
    let p: *const i32 = &n; // safe: just making a pointer
    let v = unsafe {
        // SAFETY: p came from a live &i32, so it is aligned, non-null, and valid.
        *p
    };

    println!("read {v} through {p:p}");

    let mut m = 10;
    let pm = &raw mut m;
    unsafe {
        // SAFETY: pm points at m, which is alive and not otherwise borrowed right now.
        *pm += 1;
    }

    println!("m {m}");
    let null: *const i32 = ptr::null();
    println!("null? {} as_ref {:?}", null.is_null(), unsafe { null.as_ref() });
}

// Example 2: pointer arithmetic over an array.
fn demo_pointer_math() {
    println!("-- pointer arithmetic --");

    let mut a = [1, 2, 3, 4];
    let p = a.as_mut_ptr();
    unsafe {
        // SAFETY: every offset used is in 0..4, inside the array.
        *p.add(1) = 20;

        let third = p.add(2);
        *third *= 10;
        println!("last via offset {}", *p.add(a.len() - 1));
    }

    println!("{a:?}");
    let base = a.as_ptr();
    let last = &a[3] as *const i32;
    println!("distance in elements {}", unsafe { last.offset_from(base) });
}

/// Same contract as the standard slice method: panics if mid > len.
fn split_at_mut(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = s.len();
    let p = s.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        // SAFETY: the two ranges 0..mid and mid..len do not overlap and are both in bounds,
        // so handing out two &mut slices cannot alias.
        (
            std::slice::from_raw_parts_mut(p, mid),
            std::slice::from_raw_parts_mut(p.add(mid), len - mid),
        )
    }
}

// Example 3: a safe API built on unsafe code.
fn demo_safe_abstraction() {
    println!("-- safe abstraction --");

    let mut v = [1, 2, 3, 4, 5];
    let (a, b) = split_at_mut(&mut v, 2);
    a[0] = 100;
    b[0] = 300;
    println!("{v:?}");

    // The borrow checker alone rejects the obvious version:
    // let a = &mut v[..2]; let b = &mut v[2..]; a[0] = 1; b[0] = 2;
    // error[E0499]: cannot borrow `v` as mutable more than once at a time
}

/// # Safety
///
/// `i` must be less than `s.len()`.
unsafe fn get_unchecked(s: &[u8], i: usize) -> u8 {
    // SAFETY: the caller promises i is in bounds.
    unsafe { *s.as_ptr().add(i) }
}

// Example 4: unsafe functions and their contracts.
fn demo_unsafe_fn() {
    println!("-- unsafe fn --");

    let data = b"rust";
    let c = unsafe {
        // SAFETY: 2 < 4.
        get_unchecked(data, 2)
    };

    println!("byte {}", c as char);

    // get_unchecked(data, 2);
    // error[E0133]: call to unsafe function `get_unchecked` is unsafe and requires unsafe block
    let s = unsafe {
        // SAFETY: the bytes are ASCII, hence valid UTF-8.
        std::str::from_utf8_unchecked(data)
    };

    println!("from_utf8_unchecked {s}");
}

static mut COUNTER: u32 = 0;

fn bump() -> u32 {
    unsafe {
        // SAFETY: this program only touches COUNTER from the main thread.
        COUNTER += 1;
        COUNTER
    }
}

// Example 5: static mut (prefer atomics or Mutex instead).
fn demo_static_mut() {
    println!("-- static mut --");

    bump();
    bump();
    println!("counter {}", bump());

    // Taking a reference to a static mut (&COUNTER) is denied in edition 2024, because it is
    // too easy to create aliasing &mut. Use &raw const COUNTER if you need a pointer.
    let p = &raw const COUNTER;
    println!("via raw pointer {}", unsafe { *p });
}

// Example 6: MaybeUninit.
fn demo_maybe_uninit() {
    println!("-- MaybeUninit --");

    let mut slot: MaybeUninit<i32> = MaybeUninit::uninit();
    slot.write(7);

    let n = unsafe {
        // SAFETY: we wrote a valid i32 on the line above.
        slot.assume_init()
    };

    println!("{n}");

    // Fill an array of non-Copy values one element at a time.
    let mut arr: [MaybeUninit<String>; 3] = [const { MaybeUninit::uninit() }; 3];
    for (i, cell) in arr.iter_mut().enumerate() {
        cell.write(format!("item {i}"));
    }

    let arr: [String; 3] = arr.map(|c| unsafe {
        // SAFETY: every element was written in the loop above.
        c.assume_init()
    });

    println!("{arr:?}");

    // In safe code, std::array::from_fn does the same:
    println!("{:?}", std::array::from_fn::<String, 3, _>(|i| format!("item {i}")));
}

// Example 7: a tiny fixed-capacity stack with raw memory.
struct ArrayStack<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayStack<T, N> {
    fn new() -> Self {
        Self { items: [const { MaybeUninit::uninit() }; N], len: 0 }
    }

    fn push(&mut self, t: T) -> Result<(), T> {
        if self.len == N {
            return Err(t);
        }

        self.items[self.len].write(t);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        // SAFETY: slots below the old len were initialized, and we just lowered len so this
        // slot will not be read or dropped again.
        Some(unsafe { self.items[self.len].assume_init_read() })
    }
}

impl<T, const N: usize> Drop for ArrayStack<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

fn demo_array_stack() {
    println!("-- ArrayStack --");

    let mut s: ArrayStack<String, 2> = ArrayStack::new();
    s.push("a".into()).unwrap();
    s.push("b".into()).unwrap();
    println!("push when full: {:?}", s.push("c".into()));
    println!("pop {:?} {:?} {:?}", s.pop(), s.pop(), s.pop());
}

fn main() {
    demo_raw_pointers();
    demo_pointer_math();
    demo_safe_abstraction();
    demo_unsafe_fn();
    demo_static_mut();
    demo_maybe_uninit();
    demo_array_stack();
}
