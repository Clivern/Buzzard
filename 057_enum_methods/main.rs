// 057. Enum methods
//
// impl on an enum is the same as on a struct. Methods typically match on self. You can
// also impl Default, Display, and your own traits for the enum.
//
// Run: cargo run --bin 057_enum_methods

enum Msg {
    Quit,
    Text(String),
}

impl Msg {
    fn summary(&self) -> String {
        match self {
            Msg::Quit => String::from("quit"),
            Msg::Text(s) => format!("text:{s}"),
        }
    }
}

fn main() {
    println!("{}", Msg::Quit.summary());
    println!("{}", Msg::Text(String::from("hi")).summary());
}
