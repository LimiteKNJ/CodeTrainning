use std::io::{Write, BufWriter, self};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let size = iter.next().unwrap().parse::<usize>().unwrap();
    let case = iter.next().unwrap().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let mut mat_vec = buf.split_whitespace()
    .map(|s|s.trim().parse::<isize>().unwrap())
    .collect::<Vec<_>>();
    buf.clear();

    let mut sum = 0;
    for i in 0..mat_vec.len() {
        sum += mat_vec[i];
        mat_vec[i] = sum;
    }

    for _ in 0..case {
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let x1 = iter.next().unwrap().parse::<usize>().unwrap() -1;
        let x2 = iter.next().unwrap().parse::<usize>().unwrap() -1;

        if x1 == 0 {
            write!(out, "{}\n", mat_vec[x2]).unwrap();
        } else {
            write!(out, "{}\n", mat_vec[x2] - mat_vec[x1-1]).unwrap();
        } buf.clear();
    }
}