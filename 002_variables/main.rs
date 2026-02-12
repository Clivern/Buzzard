// 002. Variables and bindings
//
// let binds a name to a value. The binding is immutable by default, so you cannot assign
// again. The type can be written after a colon, or left for the compiler to infer from
// the value. Several names can be bound in one pattern.
//
//
// You can bind a placeholder with _. It drops the value immediately and silences unused-
// variable warnings.
//
// Run: cargo run --bin 002_variables

fn main() {
    let count: i32 = 7;
    let inferred = 7;
    let (left, right) = (1, 2);
    println!("explicit {count} inferred {inferred} pair {left} {right}");
    let _unused = 0;
    println!("still have count {count}");
}
