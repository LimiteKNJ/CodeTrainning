use std::io;
use io::Write;

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut char = buf.trim().chars().collect::<Vec<_>>();
    char.sort_by(|a,b|b.cmp(a));
    buf.clear();

    let mut result = String::new();
    for c in char {
        result.push(c);
    } write!(out, "{}", result).unwrap();
}