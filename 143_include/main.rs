// 143. include_str and include_bytes
//
// include_str! copies a UTF-8 file into the binary at compile time. The path is relative
// to the current file. include_bytes! is the raw-byte version. This is the analog of
// Go's //go:embed for a single file.
//
// Run: cargo run --bin 143_include

fn main() {
    let note = include_str!("note.txt");
    println!("{note}");
}
