use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let n: i32 = s.trim().parse().unwrap();

    let mut a: Vec<String> = Vec::new();
    for _ in 0..n {
        let mut c: String = String::new();
        io::stdin().read_line(&mut c).ok();
        c = c.trim().to_string();
        if contain(&a, &c) { println!("NO") } else { println!("YES") }
        a.push(c);
    }
}

fn contain(a: &Vec<String>, c: &String) -> bool {
    for x in a {
        if x == c {
            return true;
        }
    }
    return false;
}
