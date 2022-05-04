use std::io::{Write, BufWriter, self};

fn main() {

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let input = buf.trim().split(char::is_numeric).collect::<Vec<&str>>();
    let tmp : String = input.into_iter().collect();
    buf.clear();
    
    stdin.read_line(&mut buf).unwrap();
    let find = buf.trim().to_string();
    buf.clear();

    if tmp.contains(&find) {
        write!(out, "1").unwrap();
    } else {
        write!(out, "0").unwrap();
    }
}