// 060. Option combinators
//
// map transforms Some and leaves None. and_then chains a function that itself returns
// Option (flatMap). unwrap_or supplies a default. unwrap_or_else lazily calls a
// function. filter keeps Some only when the predicate is true.
//
// Run: cargo run --bin 060_combinators

fn main() {
    let n = Some(2);
    println!("{:?}", n.map(|x| x * 10));
    println!("{:?}", n.and_then(|x| if x > 0 { Some(x) } else { None }));
    println!("{}", n.unwrap_or(0));
    println!("{:?}", n.filter(|x| *x % 2 == 0));
}
