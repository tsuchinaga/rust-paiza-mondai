use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let m: i32 = s.trim().parse().unwrap();
    if m <= 5 { println!("raw") } else if m <= 7 { println!("soft boiled") } else { println!("hard boiled") }
}
