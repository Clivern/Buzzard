// 043. Partial moves
//
// Moving one field of a struct moves that field. The other fields can still be used. The
// struct as a whole cannot, because part of it is gone. Pattern matching with ref or
// cloning the field avoids the partial move.
//
// Run: cargo run --bin 043_partial_move

struct Pair {
    name: String,
    n: i32,
}

fn main() {
    let p = Pair {
        name: String::from("ada"),
        n: 1,
    };
    let name = p.name;
    println!("{name} {}", p.n);
}
