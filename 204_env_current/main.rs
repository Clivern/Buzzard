// 204. current dir
//
// current_dir and set_current_dir change the process working directory. Relative paths
// in File::open follow it. In libraries, prefer absolute paths or paths relative to the
// caller, because changing cwd is a process-wide side effect.
//
// Run: cargo run --bin 204_env_current

fn main() {
    let cwd = std::env::current_dir().unwrap();
    println!("{}", cwd.display());
}
