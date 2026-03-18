// 157. Tests
//
// #[test] marks a function for cargo test. assert! and assert_eq! panic on failure.
// cargo test --bin 157_tests runs tests in this binary. #[should_panic] expects a panic.
// Tests in tests/ at the crate root are integration tests.
//
// Run: cargo run --bin 157_tests

fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    println!("{}", sum(&[1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn some() {
        assert_eq!(sum(&[1, 2, 3]), 6);
    }
}
