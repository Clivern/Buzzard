// 019. Printing
//
// println! writes a line. print! does not add a newline. {{ }} is a placeholder.
// {{name}} uses a named argument. {{:?}} uses Debug. {{:#?}} pretty-prints. eprint! and
// eprintln! write to stderr. format! builds a String without writing.
//
// Run: cargo run --bin 019_print

fn main() {
    let name = "Ada";
    println!("hello {name}");
    println!("debug {:?}", (1, 2));
    eprint!("note: ");
    eprintln!("on stderr");
    let s = format!("id={:04}", 7);
    println!("{s}");
}
