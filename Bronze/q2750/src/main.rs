use std::io::{Read, BufWriter};
use std::io::Write;

fn main() {
    let mut bf = String::new();
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    std::io::stdin().read_line(&mut bf).expect("Err");
    let cnt = bf.trim().parse::<usize>().expect("Err");

    let mut num_vec = Vec::new();
    for _ in 0..cnt {
        let mut bf2 = String::new();
        std::io::stdin().read_line(&mut bf2).expect("Err");
        let num = bf2.trim().parse::<isize>().expect("Err");
        
        num_vec.push(num);
    }

    num_vec.sort();

    let mut result = String::new();
    for i in 0..num_vec.len(){
        result.push_str(&num_vec[i].to_string());
        result.push('\n');
    }

    write!(out, "{}", result).expect("Err");
}