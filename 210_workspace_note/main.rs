// 210. Cargo, bins, and this repo
//
// This crate is a package with many binaries, one per lesson. cargo run --bin
// 210_workspace_note compiles only that binary plus its deps. cargo test --bin name runs
// tests in that file. rustfmt.toml at the repo root sets max_width. Add a crate under
// [dependencies] when you outgrow std.
//
// Run: cargo run --bin 210_workspace_note

fn main() {
    println!("cargo run --bin 210_workspace_note");
}
