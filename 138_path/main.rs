// 138. Path and PathBuf
//
// Path is like str: borrowed. PathBuf is like String: owned. join appends. file_name,
// extension, parent inspect parts. They are OS-native, not always UTF-8; display() is
// lossy, to_str() is Option.
//
// Run: cargo run --bin 138_path

use std::path::{Path, PathBuf};

fn main() {
    let mut p = PathBuf::from("dir1");
    p.push("dir2");
    p.push("s.log");
    let path: &Path = &p;
    println!("{} {} {}", path.parent().unwrap().display(), path.file_name().unwrap().to_string_lossy(), path.extension().unwrap().to_string_lossy());
}
