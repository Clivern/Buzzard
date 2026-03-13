// 141. Running commands
//
// std::process::Command starts another program. output() runs it and captures stdout and
// stderr. status() inherits stdio. spawn() is non-blocking. Check status.success(). This
// is the analog of os/exec.
//
// Run: cargo run --bin 141_command

use std::process::Command;

fn main() {
    let out = Command::new("echo").arg("hello").output().unwrap();
    println!("ok {} {}", out.status.success(), String::from_utf8_lossy(&out.stdout));
}
