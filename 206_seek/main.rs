// 206. Seek
//
// Seek::seek moves the cursor. SeekFrom::Start, Current, End. File and Cursor implement
// it. A pipe or stdin may not. After seek, the next read or write happens at that
// position. stream_position reports the current offset.
//
// Run: cargo run --bin 206_seek

use std::io::{Cursor, Read, Seek, SeekFrom};

fn main() {
    let mut c = Cursor::new(&b"abcdef"[..]);
    c.seek(SeekFrom::Start(3)).unwrap();
    let mut buf = [0; 2];
    c.read_exact(&mut buf).unwrap();
    println!("{}", std::str::from_utf8(&buf).unwrap());
}
