use std::io::{BufWriter, Write, self};

fn main() {
    
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();
    
    let mut result = 1;
    for i in 1..=num {
        result *= i;
    } writeln!(out, "{}", result).unwrap();
}