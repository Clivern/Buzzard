// 078. Generic functions
//
// A type parameter in angle brackets stands for a type the caller chooses. [T] is not
// enough without a bound when you need methods. T: Clone lets you call clone. The
// compiler monomorphizes: each concrete type gets its own copy of the function, so
// generics are as fast as hand-written code.
//
// Run: cargo run --bin 078_generics

fn first<T: Clone>(items: &[T]) -> Option<T> {
    items.first().cloned()
}

fn main() {
    println!("{:?} {:?}", first(&["a", "b"]), first::<i32>(&[]));
}
