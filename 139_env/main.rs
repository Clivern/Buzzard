// 139. Environment variables
//
// env::var returns Result. VarError is NotPresent or NotUnicode. var_os returns
// Option<OsString>. set_var is unsafe in recent Rust because it is racy; prefer
// configuring the child process Command instead. vars() iterates all.
//
// Run: cargo run --bin 139_env

fn main() {
    match std::env::var("HOME") {
        Ok(h) => println!("HOME={h}"),
        Err(e) => println!("no HOME: {e}"),
    }
    println!("args0 {}", std::env::args().next().unwrap());
}
