// 24. String vs &str, AsRef, Borrow, ToOwned, and Cow
//
// &str is a borrowed view of UTF-8 text; String owns a growable buffer. to_string,
// to_owned, and String::from build a String from a &str, and as_str (or &s) borrows it
// back. Prefer &str in parameters so callers can pass either.
//
// AsRef<T> is a cheap borrow-conversion: a function taking impl AsRef<Path> accepts
// &str, String, Path, and PathBuf. Borrow<T> is similar, but also promises that Eq,
// Ord, and Hash agree between the owned and borrowed forms, which is why a
// HashMap<String, V> can be queried with a &str. ToOwned goes from borrowed to owned.
//
// Cow (clone on write) is either Borrowed(&T) or Owned(T::Owned). It lets a function
// return the input untouched when nothing changed, and allocate only when it must.
//
// Run: cargo run --bin 24_asref_borrow_cow

use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::path::Path;

fn shout(s: &str) -> String {
    s.to_uppercase()
}

// Example 1: String vs &str.
fn demo_string_vs_str() {
    println!("-- String vs &str --");

    let owned = String::from("rust");
    let lit: &'static str = "hi";
    println!("{} {}", shout(lit), shout(&owned));

    let a = lit.to_string();
    let b = lit.to_owned();
    let c = String::from(lit);
    let d: String = lit.into();
    println!("{a} {b} {c} {d} -> as_str {}", owned.as_str());
    println!("String {} bytes, &str {} bytes", size_of::<String>(), size_of::<&str>());
}

fn count_chars<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().chars().count()
}

fn file_ext<P: AsRef<Path>>(p: P) -> Option<String> {
    p.as_ref().extension().map(|e| e.to_string_lossy().into_owned())
}

fn zero_out<B: AsMut<[u8]>>(mut buf: B) -> B {
    buf.as_mut().fill(0);
    buf
}

// Example 2: AsRef and AsMut for flexible parameters.
fn demo_asref() {
    println!("-- AsRef / AsMut --");

    println!("{} {}", count_chars("hi"), count_chars(String::from("🦀🦀")));
    println!(
        "{:?} {:?}",
        file_ext("a/b.rs"),
        file_ext(std::path::PathBuf::from("x.tar.gz"))
    );
    println!("{:?} {:?}", zero_out(vec![1u8, 2]), zero_out([9u8; 3]));
}

fn lookup<Q>(m: &HashMap<String, i32>, key: &Q) -> Option<i32>
where
    String: Borrow<Q>,
    Q: std::hash::Hash + Eq + ?Sized,
{
    m.get(key).copied()
}

// Example 3: Borrow lets owned-key maps accept borrowed lookups.
fn demo_borrow() {
    println!("-- Borrow --");

    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert(String::from("a"), 1);

    let k: &str = "a";
    println!("{:?} {:?}", m.get(k), m.get("missing"));
    println!("generic lookup {:?}", lookup(&m, "a"));

    let set: std::collections::HashSet<Vec<u8>> = [vec![1, 2]].into_iter().collect();
    println!("Vec<u8> set queried by slice: {}", set.contains(&[1u8, 2][..]));
}

// Example 4: ToOwned.
fn demo_to_owned() {
    println!("-- ToOwned --");

    let s: String = "hi".to_owned();
    let v: Vec<i32> = [1, 2][..].to_owned();
    let p: std::path::PathBuf = Path::new("dir").to_owned();
    println!("{s} {v:?} {}", p.display());

    // str is not Clone, so "ref".clone() only copies the &str reference (the compiler
    // warns that it does nothing). to_owned is what builds an owned String.
    let r: &str = "ref";
    let owned_r: String = r.to_owned();
    println!("to_owned of &str is a String: {owned_r}");
}

fn normalize(s: &str) -> Cow<'_, str> {
    if s.contains(' ') { Cow::Owned(s.replace(' ', "_")) } else { Cow::Borrowed(s) }
}

fn kind(c: &Cow<'_, str>) -> &'static str {
    match c {
        Cow::Borrowed(_) => "borrowed",
        Cow::Owned(_) => "owned",
    }
}

// Example 5: Cow avoids allocating when nothing changes.
fn demo_cow() {
    println!("-- Cow --");

    for input in ["ok", "a b c"] {
        let out = normalize(input);
        println!("{input:?} -> {out:?} ({})", kind(&out));
    }

    // Cow derefs to &str, so it works anywhere a &str does.
    println!("len {}", normalize("x y").len());

    // to_mut clones on first write, then reuses the owned value.
    let mut c = normalize("plain");
    c.to_mut().push_str("_edited");
    println!("after to_mut {c} ({})", kind(&c));

    let owned: String = normalize("z").into_owned();
    println!("into_owned {owned}");
}

// Example 6: Cow from std APIs, and in struct fields.
#[derive(Debug)]
struct Label<'a> {
    text: Cow<'a, str>,
}

fn demo_cow_in_std() {
    println!("-- Cow in std --");

    let good = String::from_utf8_lossy(b"fine");
    let bad = String::from_utf8_lossy(&[b'o', 0xff, b'k']);
    println!("{good} ({}) {bad} ({})", kind(&good), kind(&bad));

    let fixed = Label { text: Cow::Borrowed("static text") };
    let dynamic = Label { text: Cow::Owned(format!("item {}", 7)) };
    println!("{fixed:?} {dynamic:?}");
    println!("{} / {}", fixed.text, dynamic.text);
}

fn main() {
    demo_string_vs_str();
    demo_asref();
    demo_borrow();
    demo_to_owned();
    demo_cow();
    demo_cow_in_std();
}
