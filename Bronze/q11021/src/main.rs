use std::io;
use io::Write;

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();

    for i in 0..num {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let num = buf.split_whitespace()
                                    .map(|s|s.trim().parse().unwrap())
                                    .collect::<Vec<usize>>();
        write!(out, "Case #{}: {}\n", i+1, num[0]+num[1]);
    }
}