// 070. Default
//
// Default::default() builds a reasonable empty value. #[derive(Default)] works when
// every field is Default. unwrap_or_default uses it. Some types, such as Option, default
// to None. struct update can mix defaults with a few fields set.
//
// Run: cargo run --bin 070_default

#[derive(Default, Debug)]
struct Cfg {
    host: String,
    port: u16,
}

fn main() {
    let c = Cfg {
        port: 8080,
        ..Cfg::default()
    };
    println!("{c:?}");
}
