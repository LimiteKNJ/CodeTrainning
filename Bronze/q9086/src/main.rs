use std::io::{Write, BufWriter, self};
fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let case = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    for _ in 0..case {
        stdin.read_line(&mut buf).unwrap();
        let str = buf.trim().chars().collect::<Vec<_>>();
        buf.clear();
        write!(out, "{}{}\n", str.first().unwrap(), str.last().unwrap()).unwrap();
    }
}