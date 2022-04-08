use std::io;
use io::Write;

fn sum(n : usize) -> u128 {
    let mut result = 0;
    let mut count = 1;
    let mut iter = 1;

    while iter <= n {
        
        let mut p = 0;
        while p < count && iter <= n {
            result += count;
            iter += 1;
            p += 1;

        } count += 1;
    
    } return result;
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let m = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let n = iter.next().unwrap().trim().parse::<usize>().unwrap();

    let result = sum(n) - sum(m-1);
    write!(out, "{}", result).unwrap();
}