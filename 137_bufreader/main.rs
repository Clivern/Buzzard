// 137. BufReader and BufWriter
//
// BufReader wraps a Read and batches syscalls. lines() yields Result<String>. BufWriter
// batches writes. flush or drop the writer so the last bytes actually go out. Use them
// for anything more than a tiny file.
//
// Run: cargo run --bin 137_bufreader

use std::io::{BufRead, BufReader};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let mut path: PathBuf = std::env::temp_dir();
    path.push("learn-rust-buf.txt");
    let mut f = fs::File::create(&path).unwrap();
    writeln!(f, "one").unwrap();
    writeln!(f, "two").unwrap();
    drop(f);
    let f = fs::File::open(&path).unwrap();
    for line in BufReader::new(f).lines() {
        println!("{}", line.unwrap());
    }
    fs::remove_file(&path).unwrap();
}
