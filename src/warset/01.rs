use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let i: i32 = n.trim().parse().unwrap();
    if i >= 80 { println!("pass") } else { println!("fail") }
}
