// 080. Generic enums
//
// Option<T> and Result<T, E> are generic enums. You can write your own: Either<L, R>.
// Variants can use the type parameters independently. Inference often fills them from
// the values you construct.
//
// Run: cargo run --bin 080_generic_enums

enum Either<L, R> {
    Left(L),
    Right(R),
}

fn main() {
    let a: Either<i32, &str> = Either::Left(1);
    let b: Either<i32, &str> = Either::Right("ok");
    match (a, b) {
        (Either::Left(n), Either::Right(s)) => println!("{n} {s}"),
        _ => {}
    }
}
