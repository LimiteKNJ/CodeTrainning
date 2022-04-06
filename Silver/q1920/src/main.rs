use std::io;
use io::{Write};

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let size = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let mut vec = buf.split_whitespace()
                                .map(|s|s.trim().parse::<i32>().unwrap())
                                .collect::<Vec<_>>();
    vec.sort();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let num_input = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let ans = buf.split_whitespace()
                            .map(|s|s.trim().parse::<i32>().unwrap())
                            .collect::<Vec<_>>();
    buf.clear();

    for i in ans {
        if vec.binary_search(&i).is_ok() {
            write!(out, "1\n").unwrap();
        } else {
            write!(out, "0\n").unwrap();
        }
    }
}
