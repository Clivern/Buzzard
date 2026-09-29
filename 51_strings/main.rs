// 51. Strings: methods, UTF-8 bytes, and OsString
//
// String is an owned, growable, always-valid UTF-8 buffer; &str is a borrowed view of
// one. Most text work is a handful of methods: push, push_str, insert, replace, split,
// trim, find, contains, starts_with, to_lowercase, parse. len() counts bytes, not
// characters, and chars().count() counts Unicode scalar values, which is still not the
// same as what a user sees as one letter.
//
// Bytes and text convert explicitly. str::from_utf8 checks a byte slice and returns a
// Result; String::from_utf8 does the same for a Vec<u8> and gives the bytes back on
// failure; from_utf8_lossy replaces bad sequences with U+FFFD. as_bytes and into_bytes
// go the other way.
//
// Paths, arguments, and environment values come from the OS, which does not promise
// UTF-8. OsString and OsStr hold them unchanged; to_str() returns an Option and
// to_string_lossy() always succeeds.
//
// Run: cargo run --bin 51_strings

use std::ffi::{OsStr, OsString};
use std::fmt::Write as _;

// Example 1: building strings.
fn demo_building() {
    println!("-- building --");

    let mut s = String::from("Hello");
    s.push(',');
    s.push_str(" Rust");
    s.insert(0, '>');
    s += "!";
    println!("{s} len {} cap>=len {}", s.len(), s.capacity() >= s.len());

    let joined = ["a", "b", "c"].join("/");
    let concat = ["x", "y"].concat();
    let repeated = "ab".repeat(3);
    let formatted = format!("{}-{:03}", "id", 7);
    println!("{joined} {concat} {repeated} {formatted}");

    let mut out = String::with_capacity(32);
    for i in 0..3 {
        write!(out, "[{i}]").unwrap();
    }

    println!("{out}");
    let a = String::from("left ");
    let b = String::from("right");
    let c = a + &b; // a is moved; b is borrowed
    println!("{c} (b still here: {b})");
}

// Example 2: searching and testing.
fn demo_searching() {
    println!("-- searching --");

    let s = "The quick brown fox";
    println!(
        "contains {} starts {} ends {}",
        s.contains("quick"),
        s.starts_with("The"),
        s.ends_with('x')
    );
    println!(
        "find {:?} rfind {:?} missing {:?}",
        s.find('o'),
        s.rfind('o'),
        s.find('z')
    );
    println!("matches {}", s.matches('o').count());
    println!("match_indices {:?}", "a-b-c".match_indices('-').collect::<Vec<_>>());
    println!("eq_ignore_ascii_case {}", "RuSt".eq_ignore_ascii_case("rust"));
    println!("is_empty {} {}", "".is_empty(), " ".trim().is_empty());
}

// Example 3: splitting and trimming.
fn demo_split_trim() {
    println!("-- split / trim --");

    println!("{:?}", "a,b,,c".split(',').collect::<Vec<_>>());
    println!("{:?}", "  lots   of   space ".split_whitespace().collect::<Vec<_>>());
    println!("{:?}", "k=v=w".split_once('='));
    println!("{:?}", "k=v=w".rsplit_once('='));
    println!("{:?}", "a,b,c,d".splitn(2, ',').collect::<Vec<_>>());
    println!("{:?}", "line1\nline2\r\nline3".lines().collect::<Vec<_>>());
    println!(
        "{:?}",
        "one1two2three".split(|c: char| c.is_ascii_digit()).collect::<Vec<_>>()
    );
    println!(
        "[{}] [{}] [{}]",
        "  pad  ".trim(),
        "  pad  ".trim_start(),
        "xxpadxx".trim_matches('x')
    );
    println!("{:?} {:?}", "v1.2".strip_prefix('v'), "file.rs".strip_suffix(".rs"));
}

// Example 4: transforming.
fn demo_transform() {
    println!("-- transform --");

    println!("{} {}", "Hello".to_uppercase(), "ÉCOLE".to_lowercase());
    println!("{}", "hello".replace('l', "r"));
    println!("{}", "a-b-c".replacen('-', "+", 1));

    let reversed: String = "stressed".chars().rev().collect();
    println!("reversed {reversed}");

    let capitalized: String = {
        let mut c = "rust".chars();
        c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
    };

    println!("capitalized {capitalized}");
    let mut s = String::from("trim me!!!");
    while s.ends_with('!') {
        s.pop();
    }

    s.truncate(4);
    println!("pop/truncate {s}");

    let mut t = String::from("hello world");
    t.retain(|c| c != 'o');
    println!("retain {t}");

    let removed: String = String::from("abcdef").drain(1..3).collect();
    println!("drain {removed}");
}

// Example 5: bytes, chars, and UTF-8.
fn demo_utf8() {
    println!("-- UTF-8 --");

    let s = "naïve 🦀";
    println!("bytes {} chars {}", s.len(), s.chars().count());
    println!("as_bytes {:?}", &s.as_bytes()[..4]);
    println!(
        "char_indices {:?}",
        s.char_indices().filter(|(_, c)| !c.is_ascii()).collect::<Vec<_>>()
    );

    // Combining marks: one visible letter, two chars.
    let e_accent = "e\u{301}";
    println!(
        "{e_accent} has {} chars and {} bytes",
        e_accent.chars().count(),
        e_accent.len()
    );

    let bad_bytes: Vec<u8> = vec![0xff];
    println!(
        "{:?} {:?}",
        std::str::from_utf8(b"hi"),
        std::str::from_utf8(&bad_bytes).is_err()
    );
    println!("lossy {}", String::from_utf8_lossy(&[b'o', 0xff, b'k']));

    match String::from_utf8(vec![b'o', b'k', 0xc3]) {
        Ok(s) => println!("ok {s}"),
        Err(e) => println!(
            "bad utf8 at {}, bytes back: {:?}",
            e.utf8_error().valid_up_to(),
            e.into_bytes()
        ),
    }

    let bytes: Vec<u8> = String::from("ok").into_bytes();
    println!("into_bytes {bytes:?}");
    println!("is_char_boundary(3) in naïve: {}", s.is_char_boundary(3));
}

// Example 6: char-level processing.
fn demo_chars() {
    println!("-- char processing --");

    let text = "Hello, World! 123";
    let letters = text.chars().filter(|c| c.is_alphabetic()).count();
    let digits: u32 = text.chars().filter_map(|c| c.to_digit(10)).sum();
    println!("letters {letters} digit sum {digits}");

    let caesar: String = "abc xyz"
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (((c as u8 - b'a' + 3) % 26) + b'a') as char
            } else {
                c
            }
        })
        .collect();

    println!("caesar {caesar}");
    let is_palindrome = |s: &str| {
        let clean: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        clean.iter().eq(clean.iter().rev())
    };

    println!(
        "palindrome? {} {}",
        is_palindrome("A man, a plan, a canal: Panama"),
        is_palindrome("rust")
    );
}

// Example 7: OsStr and OsString.
fn demo_os_strings() {
    println!("-- OsStr / OsString --");

    let p = OsStr::new("file.txt");
    println!("{} {:?} len {}", p.to_string_lossy(), p.to_str(), p.len());

    let mut owned = OsString::from("dir");
    owned.push("/name");
    println!("{owned:?} into_string {:?}", owned.clone().into_string());

    for arg in std::env::args_os().take(1) {
        println!("args_os gives OsString; utf8? {}", arg.to_str().is_some());
    }

    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        let raw = OsStr::from_bytes(&[b'b', 0xff, b'd']); // not valid UTF-8, but a valid unix name
        println!("non-utf8 to_str {:?} lossy {}", raw.to_str(), raw.to_string_lossy());
    }
}

fn main() {
    demo_building();
    demo_searching();
    demo_split_trim();
    demo_transform();
    demo_utf8();
    demo_chars();
    demo_os_strings();
}
