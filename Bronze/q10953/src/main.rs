use std::io;

use io::Write;

fn main() {
    let mut bf = String::new();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    std::io::stdin().read_line(&mut bf).expect("Err");
    let cnt = bf.trim().parse::<usize>().expect("Err");

    for _ in 0..cnt {
        let mut bf = String::new();
        std::io::stdin().read_line(&mut bf).expect("Err");
        let num = bf.split(',')
                                .map(|s|s.trim().parse().expect("Err"))
                                .collect::<Vec<usize>>();
        writeln!(out, "{}", num[0]+num[1]);
    }
}