use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let n: i32 = s.trim().parse().unwrap();
    for _ in 0..n {
        let mut t = String::new();
        io::stdin().read_line(&mut t).ok();
        match t.trim() {
            "forward" => println!("Sunny"),
            "reverse" => println!("Rainy"),
            "sideways" => println!("Cloudy"),
            _ => {}
        }
    }
}
