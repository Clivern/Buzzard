// 156. cfg
//
// #[cfg(test)] compiles the item only for tests. cfg(unix), cfg(windows), cfg(target_os
// = "macos") pick platforms. cfg(feature = "foo") is Cargo features. cfg! is a boolean
// in code. Use it instead of commenting out blocks.
//
// Run: cargo run --bin 156_cfg

fn main() {
    if cfg!(unix) {
        println!("unix");
    } else {
        println!("not unix");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn always() {
        assert!(true);
    }
}
