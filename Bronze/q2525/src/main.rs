use std::io::{Write, BufWriter, self};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let mut hour = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let mut min = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let exc = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    min += exc;
    if min >= 60 { hour += min / 60; min = min % 60; }
    if hour >= 24 { hour -= 24; }
    write!(out, "{} {}", hour, min).unwrap();
}