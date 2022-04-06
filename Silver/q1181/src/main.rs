use std::io;
use io::Write;

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    
    let mut str_vec : Vec<String> = Vec::new();
    for _ in 0..num {
        stdin.read_line(&mut buf).unwrap();
        str_vec.push(buf.trim().to_string());
        buf.clear();
    }

    str_vec.sort_by(|a, b| a.cmp(&b));
    str_vec.sort_by(|a, b| a.len().cmp(&b.len()));
    str_vec.dedup();
    
    for str in str_vec {
        writeln!(out, "{}", str).unwrap();
    }
}