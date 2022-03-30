use std::io;
use io::Write;

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut numV : Vec<usize> = Vec::new();

    for i in 0..2 {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<usize>().unwrap();
        numV.push(num);
    }

    write!(out, "{}", numV[0]+numV[1]);
}