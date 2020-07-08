use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let i: i32 = s.trim().parse().unwrap();
    println!("{}", i - 24)
}
