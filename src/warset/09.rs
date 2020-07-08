use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let nm: Vec<&str> = s.trim().split_whitespace().collect();
    let n: usize = nm[0].trim().parse().unwrap();
    let m: usize = nm[1].trim().parse().unwrap();

    let mut dots: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        let mut l = String::new();
        io::stdin().read_line(&mut l).ok();
        let sl: Vec<char> = l.trim().chars().collect();
        dots.push(sl)
    }

    let mut res: Vec<String> = Vec::new();
    if vertical(&n, &m, &dots) || horizontal(&n, &m, &dots) {
        res.push("line".to_string())
    }
    if point(&n, &m, &dots) {
        res.push("point".to_string())
    }

    if res.len() > 0 {
        println!("{} symmetry", res.join(" "))
    } else {
        println!("none")
    }
}

// 縦線で線対称
fn vertical(n: &usize, m: &usize, dots: &Vec<Vec<char>>) -> bool {
    let &n = n;
    let &m = m;
    for i in 0..n {
        for j in 0..(m / 2) {
            if dots[i][j] != dots[i][m - j - 1] {
                return false;
            }
        }
    }
    return true;
}

// 横線で線対称
fn horizontal(n: &usize, m: &usize, dots: &Vec<Vec<char>>) -> bool {
    let &n = n;
    let &m = m;
    for i in 0..(n / 2) {
        for j in 0..m {
            if dots[i][j] != dots[n - i - 1][j] {
                return false;
            }
        }
    }
    return true;
}

// 中心の点で点対象
fn point(n: &usize, m: &usize, dots: &Vec<Vec<char>>) -> bool {
    let &n = n;
    let &m = m;
    for i in 0..n {
        for j in 0..m {
            if dots[i][j] != dots[n - i - 1][m - j - 1] {
                return false;
            }
        }
    }
    return true;
}
