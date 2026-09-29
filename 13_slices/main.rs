// 13. Slices and string slices
//
// A slice &[T] is a view into a run of elements owned by something else: a pointer and
// a length. &v[1..4] borrows that span. Because the length travels with the pointer,
// &[T] is a "fat" pointer, twice the size of a plain one.
//
// &str is a slice of UTF-8 bytes. String is the owned, growable buffer behind it, and a
// string literal is a &'static str. Byte ranges on &str must land on char boundaries or
// they panic; chars(), char_indices(), and get() are safer when the text is not ASCII.
//
// Take &[T] and &str in function parameters: callers can then pass arrays, Vecs,
// Strings, and literals.
//
// Run: cargo run --bin 13_slices

fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// Example 1: slicing arrays and Vecs.
fn demo_slices() {
    println!("-- slices --");

    let v = vec![1, 2, 3, 4, 5];
    println!("{} {:?} {:?} {:?}", sum(&v[1..4]), &v[..2], &v[3..], &v[..]);

    let arr = [10, 20, 30];
    println!("array and vec both coerce: {} {}", sum(&arr), sum(&v));
    println!("fat pointer bytes {} thin {}", size_of::<&[i32]>(), size_of::<&i32>());
    println!("empty {:?} len {}", &v[2..2], v[2..2].len());
}

// Example 2: slice methods.
fn demo_slice_methods() {
    println!("-- slice methods --");

    let s = [3, 1, 4, 1, 5, 9, 2, 6];
    println!("first {:?} last {:?} get {:?}", s.first(), s.last(), s.get(100));
    println!(
        "contains 9? {} position of 1 {:?}",
        s.contains(&9),
        s.iter().position(|&n| n == 1)
    );
    println!("starts_with {} ends_with {}", s.starts_with(&[3, 1]), s.ends_with(&[6]));

    let (head, tail) = s.split_first().unwrap();
    println!("head {head} tail {tail:?}");

    let parts: Vec<_> = s.split(|&n| n == 1).collect();
    println!("split on 1 {parts:?}");
    println!("chunks_exact {:?}", s.chunks_exact(3).collect::<Vec<_>>());
    println!(
        "concat {:?} join {:?}",
        [[1, 2], [3, 4]].concat(),
        ["a", "b", "c"].join("-")
    );
}

// Example 3: mutable slices change the owner's data.
fn demo_mut_slices() {
    println!("-- mut slices --");

    let mut v = vec![5, 4, 3, 2, 1];
    let middle = &mut v[1..4];
    middle.sort();
    println!("{v:?}");
    v[..2].fill(0);
    v.swap(0, 4);
    println!("{v:?}");
    v.rotate_left(1);
    println!("rotated {v:?}");

    let mut buf = [0u8; 4];
    buf[1..3].copy_from_slice(&[7, 8]);
    println!("copied {buf:?}");
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// Example 4: &str and String.
fn demo_str_slices() {
    println!("-- str slices --");

    let owned = String::from("hello rust");
    println!("{} {}", first_word(&owned), first_word("only"));

    let literal: &'static str = "rustacean";
    println!("{} {}", &literal[0..4], &literal[4..]);

    let as_slice: &str = owned.as_str();
    println!(
        "as_str {as_slice} same bytes? {}",
        std::ptr::eq(as_slice.as_ptr(), owned.as_ptr())
    );
}

// Example 5: char boundaries.
fn demo_char_boundaries() {
    println!("-- char boundaries --");

    let s = "héllo";
    println!("bytes {} chars {}", s.len(), s.chars().count());
    println!("is_char_boundary(2)? {}", s.is_char_boundary(2));

    // &s[0..2] would panic: byte index 2 is inside 'é'.
    println!("get(0..2) {:?} get(0..3) {:?}", s.get(0..2), s.get(0..3));

    for (i, c) in s.char_indices() {
        print!("{i}:{c} ");
    }

    println!();

    // The first n chars, done safely.
    let first3: String = s.chars().take(3).collect();
    println!("first 3 chars {first3}");
}

// Example 6: slicing into structured text.
fn demo_parsing_with_slices() {
    println!("-- parsing --");

    let line = "key=value; other=42";
    for part in line.split(';') {
        if let Some((k, v)) = part.trim().split_once('=') {
            println!("{k} -> {v}");
        }
    }

    let path = "/usr/local/bin";
    println!("strip {:?} {:?}", path.strip_prefix("/usr"), path.strip_suffix("/lib"));

    let ext = "photo.tar.gz".rsplit_once('.').map(|(_, e)| e);
    println!("ext {ext:?}");
}

fn main() {
    demo_slices();
    demo_slice_methods();
    demo_mut_slices();
    demo_str_slices();
    demo_char_boundaries();
    demo_parsing_with_slices();
}
