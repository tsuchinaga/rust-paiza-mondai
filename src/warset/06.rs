use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let ss: Vec<&str> = s.trim().split_whitespace().collect();
    println!("{}.{}.", ss[0].chars().nth(0).unwrap(), ss[1].chars().nth(0).unwrap())
}
