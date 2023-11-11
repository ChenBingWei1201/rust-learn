
// Fix all errors without adding newline
fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s += " world";
    s += "!";

    println!("{}", s);
}
