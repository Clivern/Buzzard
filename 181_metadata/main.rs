// 181. File metadata
//
// metadata() returns type, len, modified time, permissions. exists() is a convenience
// that still races: the file can vanish between check and open. Prefer opening and
// handling NotFound. Unix has extra fields behind std::os::unix.
//
// Run: cargo run --bin 181_metadata

fn main() {
    let meta = std::fs::metadata("Cargo.toml").unwrap();
    println!("len {} file {}", meta.len(), meta.is_file());
}
