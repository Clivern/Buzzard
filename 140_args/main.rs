// 140. Command-line args
//
// env::args() is an iterator of String, starting with the program name. args_os() is
// OsString for non-UTF-8. For flags, a crate such as clap is usual; std has nothing like
// Go's flag package. Skip(1) drops the program name.
//
// Run: cargo run --bin 140_args

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    println!("argc {} {:?}", args.len(), args);
}
