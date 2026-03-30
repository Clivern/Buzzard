// 205. Read and Write
//
// Read::read fills a buffer and returns the byte count. read_to_end and read_to_string
// are helpers. Write::write may write only part of the buffer; write_all loops until
// done. Both traits are implemented for files, sockets (on some platforms), Cursor, and
// stdio.
//
// Run: cargo run --bin 205_read_write_trait

use std::io::{Read, Write};

fn main() {
    let mut buf = Vec::new();
    let mut src: &[u8] = b"abc";
    src.read_to_end(&mut buf).unwrap();
    let mut out: Vec<u8> = Vec::new();
    out.write_all(&buf).unwrap();
    println!("{}", String::from_utf8(out).unwrap());
}
