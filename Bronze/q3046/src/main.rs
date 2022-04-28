use std::io::{Write, BufWriter, self};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let num1 = iter.next().unwrap().trim().parse::<isize>().unwrap();
    let avg = iter.next().unwrap().trim().parse::<isize>().unwrap();

    write!(out, "{}", avg*2 - num1).unwrap();
}