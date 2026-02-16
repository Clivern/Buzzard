// 051. Struct update syntax
//
// ..other copies the remaining fields from another instance. Fields you write first
// override. This moves fields that are not Copy, so other may be partially moved
// afterward.
//
// Run: cargo run --bin 051_update

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    tls: bool,
}

fn main() {
    let base = Config {
        host: String::from("localhost"),
        port: 8080,
        tls: false,
    };
    let prod = Config {
        host: String::from("api.example.com"),
        tls: true,
        ..base
    };
    println!("{prod:?}");
}
