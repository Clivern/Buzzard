// 180. Directories
//
// create_dir, create_dir_all, read_dir, remove_dir_all are in std::fs. read_dir yields
// Result<DirEntry>. metadata tells you if a path is a file. Walks of large trees usually
// use the walkdir crate; std is enough for one level.
//
// Run: cargo run --bin 180_fs_dir

use std::fs;

fn main() {
    let dir = std::env::temp_dir().join("learn-rust-dir");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("a.txt"), b"a").unwrap();
    for e in fs::read_dir(&dir).unwrap() {
        let e = e.unwrap();
        println!("{}", e.file_name().to_string_lossy());
    }
    fs::remove_dir_all(&dir).unwrap();
}
