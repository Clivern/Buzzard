// 059. Result
//
// Result<T, E> is Ok(T) or Err(E). A function that can fail returns Result. ? returns
// the Err early from a function that itself returns Result. () as T means success with
// no value.
//
// Run: cargo run --bin 059_result

fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    s.parse()
}

fn main() {
    println!("{:?} {:?}", parse_port("8080"), parse_port("no"));
}
