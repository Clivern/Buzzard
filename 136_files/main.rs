// 136. Files
//
// File::create opens a file for writing. write_all writes bytes. File::open reads.
// std::fs::read reads a whole file into Vec<u8>. std::fs::read_to_string reads UTF-8.
// Always handle the io::Error. tempfile-like names can use env::temp_dir.
//
//
// write writes bytes. Write::write_all retries until the whole buffer is written or an
// error is returned.
//
// Run: cargo run --bin 136_files

use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let mut path: PathBuf = std::env::temp_dir();
    path.push("learn-rust-files.txt");
    let mut f = fs::File::create(&path).unwrap();
    f.write_all(b"hello file\n").unwrap();
    drop(f);
    let s = fs::read_to_string(&path).unwrap();
    println!("{s}");
    fs::remove_file(&path).unwrap();
    println!("wrote {} bytes", b"hello file\n".len());
}
