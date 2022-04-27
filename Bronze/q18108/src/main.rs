use std::io::{Write, BufWriter, self};
fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    write!(out, "{}", buf.trim().parse::<usize>().unwrap() - 543).unwrap();
    buf.clear();
}
