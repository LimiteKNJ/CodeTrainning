use std::io;
use io::Write;

fn is_prime(n: usize) -> bool {

    let limit = (n as f64).sqrt() as usize;
    if n <= 1 {
        return false;

    } else {
        for i in 2..=limit {
            if n % i == 0 {
                return false;
            }
        } return true;
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut num = buf.split_whitespace()
                    .map(|s|s.trim().parse().unwrap())
                    .collect::<Vec<usize>>();

    for i in num[0]..num[1]+1 {
        if is_prime(i) { write!(out, "{}\n", i).unwrap()}
    }
}