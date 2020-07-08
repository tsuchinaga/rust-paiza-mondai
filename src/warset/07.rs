use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let n: i32 = s.trim().parse().unwrap();
    let mut a = Vec::new();
    for _ in 0..n {
        let mut t: String = String::new();
        io::stdin().read_line(&mut t).ok();
        let m: i32 = t.trim().parse().unwrap();
        if m % 2 == 1 {
            a.push(m);
        }
    }
    a.sort();
    for x in a {
        println!("{}", x);
    }
}
