// 04. Tuples and arrays
//
// A tuple groups a fixed number of values of mixed types: (i32, &str). Read a field
// with a dot and an index, pair.0, or destructure the whole thing with let. () is the
// unit tuple, the type of an expression with no useful value. A one-element tuple needs
// a trailing comma: (1,).
//
// An array has a fixed length that is part of its type: [i32; 3]. All elements share
// one type. [0; 4] repeats a value. Arrays live inline (on the stack for a local), and
// out-of-range indexing panics. &a[1..] borrows part of an array as a slice.
//
// Run: cargo run --bin 04_tuples_arrays

// Example 1: building, reading, and destructuring tuples.
fn demo_tuples() {
    println!("-- tuples --");

    let pair = (7, "ok");
    println!("{} {}", pair.0, pair.1);

    let (n, label) = pair;
    println!("unpacked {n} {label}");

    let nested = ((1, 2), [3, 4], "five");
    let ((a, _), [_, d], e) = nested;
    println!("nested {} picked {a} {d} {e}", (nested.0).1);

    let single = (1,);

    // Without the comma, (1) is just 1 in parentheses.
    println!("single {single:?} len-one tuple field {}", single.0);

    let unit = ();
    println!("unit {unit:?}");
}

fn min_max(nums: &[i32]) -> (i32, i32) {
    let mut lo = nums[0];
    let mut hi = nums[0];
    for &n in nums {
        lo = lo.min(n);
        hi = hi.max(n);
    }

    (lo, hi)
}

// Example 2: tuples as multiple return values, and swapping.
fn demo_tuple_returns() {
    println!("-- tuple returns --");

    let (lo, hi) = min_max(&[4, -2, 9, 3]);
    println!("lo {lo} hi {hi}");

    let mut a = 1;
    let mut b = 2;
    (a, b) = (b, a);
    println!("swapped a {a} b {b}");

    // Tuples compare element by element, left to right.
    println!("(1, 9) < (2, 0)? {}", (1, 9) < (2, 0));
    println!("(\"a\", 2) == (\"a\", 3)? {}", ("a", 2) == ("a", 3));
}

// Example 3: building and reading arrays.
fn demo_arrays() {
    println!("-- arrays --");

    let a: [i32; 3] = [1, 2, 3];
    let zeros = [0; 4];
    let grid = [[0u8; 3]; 2];
    println!("first {} len {} zeros {zeros:?} grid {grid:?}", a[0], a.len());

    let mut b = a;
    b[0] = 100;
    println!("copy changed {b:?}, original {a:?}");

    // get returns None instead of panicking on a bad index.
    println!("get {:?} {:?}", a.get(1), a.get(10));

    // let oops = a[10];
    // error: this operation will panic at runtime: index out of bounds

    println!("contains 2? {} iter sum {}", a.contains(&2), a.iter().sum::<i32>());
    let squares: [u32; 5] = std::array::from_fn(|i| (i as u32) * (i as u32));
    println!("from_fn {squares:?}");

    let doubled = a.map(|n| n * 2);
    println!("map {doubled:?}");
}

// Example 4: arrays and slices.
fn demo_array_slices() {
    println!("-- array slices --");

    let a = [10, 20, 30, 40, 50];
    let view: &[i32] = &a[1..4];
    println!("view {view:?} len {}", view.len());
    println!("first {:?} last {:?}", a.first(), a.last());

    let (left, right) = a.split_at(2);
    println!("split {left:?} {right:?}");

    for pair in a.windows(2) {
        print!("{} ", pair[1] - pair[0]);
    }

    println!();
    for chunk in a.chunks(2) {
        print!("{chunk:?} ");
    }

    println!();

    let mut m = [3, 1, 2];
    m.sort();
    m.reverse();
    println!("sorted desc {m:?}");
}

// Example 5: iterating over arrays.
fn demo_array_iteration() {
    println!("-- array iteration --");

    let names = ["ann", "ben", "cy"];
    for name in names {
        print!("{name} ");
    }

    println!();
    for (i, name) in names.iter().enumerate() {
        print!("{i}:{name} ");
    }

    println!();
    let mut nums = [1, 2, 3];
    for n in &mut nums {
        *n *= 10;
    }

    println!("{nums:?}");
}

fn main() {
    demo_tuples();
    demo_tuple_returns();
    demo_arrays();
    demo_array_slices();
    demo_array_iteration();
}
