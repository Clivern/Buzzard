// 077. Cow
//
// Cow<'_, T> is clone-on-write: Borrowed(&T) or Owned(T::Owned). You can return a borrow
// when no change is needed, and only allocate when you must mutate. into_owned always
// yields the owned form. It is common in parsers and APIs that sometimes intern and
// sometimes construct.
//
// Run: cargo run --bin 077_cow

use std::borrow::Cow;

fn normalize(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))
    } else {
        Cow::Borrowed(s)
    }
}

fn main() {
    println!("{} {}", normalize("ok"), normalize("a b"));
}
