// 056. Enums
//
// An enum is a value that is one of several variants. Each variant can hold data. match
// (or if let) is how you read that data. Unlike a C enum, variants are not just integers
// unless you assign discriminants and they have no fields.
//
//
// match is how you recover the data inside a variant. Wildcards are a last resort;
// prefer naming every variant you care about.
//
// Run: cargo run --bin 056_enums

#[derive(Debug)]
enum Ip {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = Ip::V4(127, 0, 0, 1);
    let loopback = Ip::V6(String::from("::1"));
    println!("{home:?} {loopback:?}");
    match home {
        Ip::V4(a, b, c, d) => println!("{a}.{b}.{c}.{d}"),
        Ip::V6(s) => println!("{s}"),
    }
}
