// 182. Cursor
//
// io::Cursor wraps an in-memory buffer as Read + Write + Seek. Tests use it instead of
// real files. Cursor::new(Vec::new()) is a growable writer. into_inner() takes the
// buffer back.
//
// Run: cargo run --bin 182_cursor

use std::io::{Cursor, Read, Seek, SeekFrom, Write};

fn main() {
    let mut c = Cursor::new(Vec::new());
    c.write_all(b"hello").unwrap();
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut s = String::new();
    c.read_to_string(&mut s).unwrap();
    println!("{s}");
}
