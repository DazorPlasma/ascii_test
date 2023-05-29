use ascii::AsciiStr;
fn main() {
    let s = AsciiStr::from_ascii("asdfghjkl").unwrap();
    for chr in s.as_bytes() {
        print!("{} ", *chr as char);
    }
    println!("");
    for chr in s.chars() {
        print!("{} ", chr);
    }
    println!("");
}