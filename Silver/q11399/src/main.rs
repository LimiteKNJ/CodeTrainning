use std::io;
use io::Write;

fn main() {
    
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let cnt = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut people_wait : Vec<usize> = vec![0; cnt];
    stdin.read_line(&mut buf).unwrap();
    people_wait = buf.split_whitespace()
                        .map(|x|x.trim().parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
    people_wait.sort();

    let mut result = 0;
    let mut waiting = 0;
    for i in people_wait {
        waiting += i;
        result += waiting;

    } write!(out, "{}", result).unwrap();
}