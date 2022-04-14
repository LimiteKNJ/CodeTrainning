use std::io::{Read, Write, BufWriter, self};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let case = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut num_vec = Vec::new();
    for _ in 0..case {
        stdin.read_line(&mut buf).unwrap();
        let num = buf.trim().to_string();
        num_vec.push(num);
        buf.clear();
    }

    for i in num_vec {
        let tmp = i.trim().chars().collect::<Vec<_>>();
        if tmp[tmp.len()-1].to_digit(10).unwrap() % 2 == 0 {
            write!(out, "even\n").unwrap();
        } else {
            write!(out, "odd\n").unwrap();
        }
    }
}