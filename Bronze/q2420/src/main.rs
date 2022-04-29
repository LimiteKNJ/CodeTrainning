use std::io::{Write, BufWriter, self};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let po1 = iter.next().unwrap().parse::<i128>().unwrap();
    let po2 = iter.next().unwrap().parse::<i128>().unwrap();
    write!(out, "{}", (po1 - po2).abs()).unwrap();
}