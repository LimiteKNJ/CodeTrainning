use std::io;
use io::Write;

fn combine (n : usize, k : usize) -> usize {

    let mut numuric = 1;
    let mut delimi = 1;
    let mut iter = 0;

    while iter < k {
        numuric *= n - iter;
        iter += 1;
        delimi *= iter;
    }

    return numuric / delimi;
}

fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).unwrap();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let num : Vec<usize> = bf.split_whitespace()
                            .map(|s|s.trim().parse().unwrap())
                            .collect::<Vec<_>>();

    writeln!(out, "{}", combine(num[0], num[1]));
}