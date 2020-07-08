use std::io;

fn main() {
    let mut s = String::new();
    let mut t = String::new();
    io::stdin().read_line(&mut s).ok();
    io::stdin().read_line(&mut t).ok();
    if s.trim() == t.trim() { println!("YES") } else { println!("NO") }
}
