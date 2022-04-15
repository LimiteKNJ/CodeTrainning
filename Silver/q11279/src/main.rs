use std::io::{Read, Write, self, BufWriter};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let size = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut heap = BinaryHeap::new();
    for _ in 0..size {
        stdin.read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<usize>().unwrap();

        if num == 0 {
            let pop = heap.pop();
            match pop {
                Some(n) => write!(out,"{}\n",n).unwrap(),
                None => write!(out,"0\n").unwrap()
            }
        } else {
            heap.push(num);
        }
        buf.clear();
    }
}