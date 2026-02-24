// 075. AsRef and AsMut
//
// AsRef<T> is a cheap conversion to &T. Functions take impl AsRef<Path> so they accept
// Path, PathBuf, str, and String. AsMut is the mutable counterpart. It is about
// borrowing, not taking ownership; Into is for ownership transfer.
//
// Run: cargo run --bin 075_asref

fn count_chars<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().chars().count()
}

fn main() {
    println!("{} {}", count_chars("hi"), count_chars(String::from("🦀")));
}
