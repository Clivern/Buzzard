// 159. Declarative macros
//
// macro_rules! defines a macro by matching tokens. $x:expr is an expression.
// $($x:expr),* repeats. Macros expand before type checking, which is why println! can
// take a variable number of arguments. Prefer functions when a function will do; macros
// are for syntax you cannot write as a function.
//
//
// The same macro can be called from several places. Expansion happens before type
// checking, so add!(2, 3) becomes 2 + 3.
//
// Run: cargo run --bin 159_macros

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    println!("{}", add!(2, 3));
    println!("{}", add!(10, 20));
}
