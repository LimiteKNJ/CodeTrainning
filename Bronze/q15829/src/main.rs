use std::io;
use io::Write;
fn main() {
    let mut buf = String::new();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    std::io::stdin().read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let key = buf.trim().chars().collect::<Vec<char>>();

    let mut multiple = 1;
    let mut hash = 0;
    for i in 0..key.len() {
        hash += (key[i] as u128 - 'a' as u128 + 1) * multiple;
        multiple.overflowing_mul(multiple * 31);
    } 
    writeln!(out, "{}", hash);
}