// 142. stdin stdout stderr
//
// io::stdin().read_line reads one line into a String. stdout() is a handle; lock it if
// many threads write. Write is implemented for File, Vec<u8>, Cursor, and the stdio
// handles. use std::io::Write to get write_all and writeln!.
//
// Run: cargo run --bin 142_stdin

use std::io::{self, Write};

fn main() {
    let mut out = io::stdout().lock();
    writeln!(out, "stdout line").unwrap();
    writeln!(io::stderr(), "stderr line").unwrap();
}
