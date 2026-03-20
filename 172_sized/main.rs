// 172. Sized
//
// T is Sized by default: the size is known at compile time. ?Sized allows unsized types
// such as str, [T], and dyn Trait. They must live behind a pointer. Box<T> where T:
// ?Sized is how Box<str> and Box<dyn Error> work.
//
// Run: cargo run --bin 172_sized

fn name_len(s: &str) -> usize {
    s.len()
}

fn main() {
    let boxed: Box<str> = "hello".into();
    println!("{} {}", name_len(&boxed), std::mem::size_of_val(&*boxed));
}
