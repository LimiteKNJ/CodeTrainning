use std::io::{Read, Write, BufWriter, self};

fn main() {
    let mut stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut sum = 0;
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let num = buf.replace("\r","").replace("\n", "").split(',')
                        .map(|s|match s.trim().parse::<usize>(){
                            Ok(n) => {n}, Err(e) => {0},
                        }).collect::<Vec<usize>>();
    sum = num.iter().sum::<usize>();
    write!(out,"{}", sum).unwrap(); 
    buf.clear();
}